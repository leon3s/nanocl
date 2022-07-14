use uuid::Uuid;
use ntex::{web, rt};
use ntex::util::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use futures::channel::mpsc;
use futures::{StreamExt, SinkExt, stream};

use crate::services;
use crate::models::Pool;
use crate::config::DaemonConfig;

#[derive(Debug, Clone)]
pub struct EventSystemClient {
  pub(crate) id: Uuid,
  pub(crate) sender: mpsc::UnboundedSender<Bytes>,
}

impl EventSystemClient {
  pub fn new(sender: mpsc::UnboundedSender<Bytes>) -> Self {
    EventSystemClient {
      id: Uuid::new_v4(),
      sender,
    }
  }
}

type EventSystemClients = HashMap<Uuid, EventSystemClient>;

#[derive(Clone)]
pub struct EventSystemConfig {
  config: DaemonConfig,
  // Todo watch docker_api event
  #[allow(dead_code)]
  docker_api: web::types::State<bollard::Docker>,
  pool: web::types::State<Pool>,
  clients: Arc<Mutex<EventSystemClients>>,
}

#[derive(Clone)]
pub struct EventSystem(EventSystemConfig);

#[derive(Debug, Clone)]
pub enum EventMessage {
  /// Connect to real time nginx logs
  NginxLog(EventSystemClient),
}

fn unlock_mutex<T>(mutex: &'_ Arc<Mutex<T>>) -> Option<MutexGuard<'_, T>> {
  match mutex.lock() {
    Err(err) => {
      log::error!("Unable to lock clients mutex {:#?}", err);
      None
    }
    Ok(guard) => Some(guard),
  }
}

impl EventSystem {
  pub fn new(args: EventSystemConfig) -> Self {
    EventSystem(args)
  }

  pub fn start_tasks(&self) {
    let state_dir = self.0.config.state_dir.clone();
    let pool = self.0.pool.clone();
    let clients = self.0.clients.clone();
    let mut receiver = services::nginx::watch_nginx_logs(state_dir, pool);
    rt::Arbiter::new().exec_fn(move || {
      rt::spawn(async move {
        while let Some(data) = receiver.next().await {
          let iter: EventSystemClients =
            if let Some(clients) = unlock_mutex(&clients) {
              println!("clients length {}", &clients.len());
              let iter = clients.clone();
              drop(clients);
              iter
            } else {
              HashMap::new()
            };
          let mut stream = stream::iter(iter);
          while let Some((id, mut client)) = stream.next().await {
            let data = serde_json::to_string(&data).unwrap();
            if client.sender.is_closed() {
              if let Some(mut clients_guard) = unlock_mutex(&clients) {
                clients_guard.remove(&id);
              }
            } else if let Err(err) = client.sender.send(Bytes::from(data)).await
            {
              log::info!("Unable to send nginx log to client {:#?}", err);
            }
          }
        }
        rt::Arbiter::current().stop();
      });
    });
  }

  pub async fn handle_events(&mut self, event: EventMessage) {
    match event {
      EventMessage::NginxLog(client) => {
        if let Some(mut clients) = unlock_mutex(&self.0.clients) {
          log::info!("added new client in event system");
          clients.insert(client.id, client);
          drop(clients);
        }
      }
    }
  }
}

/// Start background event system
/// This allow us to have a global class to have clients to send events
pub async fn start(
  config: DaemonConfig,
  docker_api: bollard::Docker,
  pool: Pool,
) -> mpsc::UnboundedSender<EventMessage> {
  let (tx, mut rx) = mpsc::unbounded();
  let mut system = EventSystem::new(EventSystemConfig {
    config,
    docker_api: web::types::State::new(docker_api),
    pool: web::types::State::new(pool),
    clients: Arc::new(Mutex::new(HashMap::new())),
  });
  system.start_tasks();
  rt::Arbiter::new().exec_fn(move || {
    rt::spawn(async move {
      println!("break");
      while let Some(event) = rx.next().await {
        system.handle_events(event).await;
      }

      log::error!("Background system died");
      rt::Arbiter::current().stop();
    });
  });

  tx
}
