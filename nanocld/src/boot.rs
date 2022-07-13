//! File used to describe daemon boot
use ntex::web;
// use std::sync::mpsc::channel;
// use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher, Op};

use bollard::errors::Error as DockerError;

use crate::config::DaemonConfig;
use crate::{services, repositories};
use crate::models::{Pool, NamespacePartial};

use crate::errors::DaemonError;

embed_migrations!("./migrations");

#[derive(Clone)]
pub struct BootState {
  pub(crate) pool: Pool,
  pub(crate) docker_api: bollard::Docker,
}

/// # Create default namespace
/// Create a namespace with default as name if he doesn't exist
///
/// # Arguments
/// - [pool](web::types::State<Pool>) Postgres database pool
///
/// # Examples
/// ```rust,norun
/// create_default_nsp(&pool).await;
/// ```
async fn create_default_nsp(
  pool: &web::types::State<Pool>,
) -> Result<(), DaemonError> {
  const NSP_NAME: &str = "global";
  match repositories::namespace::inspect_by_name(NSP_NAME.to_string(), pool)
    .await
  {
    Err(_err) => {
      let new_nsp = NamespacePartial {
        name: NSP_NAME.to_string(),
      };
      repositories::namespace::create(new_nsp, pool).await?;
      Ok(())
    }
    Ok(_) => Ok(()),
  }
}

pub async fn create_default_network(
  docker: &bollard::Docker,
) -> Result<(), DockerError> {
  let network_name = "nanocl";
  let state = services::utils::get_network_state(docker, network_name).await?;
  if state == services::utils::NetworkState::NotFound {
    services::utils::create_network(docker, network_name).await?;
  }
  Ok(())
}

async fn boot_docker_services(
  config: &DaemonConfig,
  docker: &bollard::Docker,
) -> Result<(), DaemonError> {
  create_default_network(docker).await?;
  // Boot postgresql service to ensure database connection
  services::postgresql::boot(config, docker).await?;
  // Boot dnsmasq service to manage domain names
  services::dnsmasq::boot(config, docker).await?;
  // Boot nginx service to manage proxy
  services::nginx::boot(config, docker).await?;
  Ok(())
}

/// Boot function called before http server start to
/// initialize his state and some background task
pub async fn boot(
  config: &DaemonConfig,
  docker_api: &bollard::Docker,
) -> Result<BootState, DaemonError> {
  log::info!("booting");
  boot_docker_services(config, docker_api).await?;
  let postgres_ip = services::postgresql::get_postgres_ip(docker_api).await?;
  log::info!("creating postgresql state pool");
  // Connect to postgresql
  let db_pool = services::postgresql::create_pool(postgres_ip.to_owned());
  let pool = web::types::State::new(db_pool.to_owned());
  let conn = services::postgresql::get_pool_conn(&pool)?;
  log::info!("runing migration");
  // wrap into state to be abble to use our functions
  embedded_migrations::run(&conn)?;
  // Create default namesapce
  create_default_nsp(&pool).await?;

  // Todo move this to tasks
  // ntex::rt::spawn(async move {
  //   // Create a channel to receive the events.
  //   let (tx, rx) = channel();
  //   // Create a watcher object, delivering raw events.
  //   // The notification back-end is selected based on the platform.
  //   let mut watcher = raw_watcher(tx).unwrap();
  //   // Add a path to be watched. All files and directories at that path and
  //   // below will be monitored for changes.
  //   watcher
  //     .watch("/var/lib/nanocl/nginx/log", RecursiveMode::Recursive)
  //     .unwrap();
  //   loop {
  //     match rx.recv() {
  //       Ok(RawEvent {
  //         path: Some(path),
  //         op: Ok(op),
  //         cookie,
  //       }) => {
  //         log::debug!("watcher event {:?} {:?} ({:?})", op, path, cookie);
  //         if path.to_string_lossy() != "/var/lib/nanocl/nginx/log/access.log" {
  //           return;
  //         }
  //         if op == Op::WRITE {
  //           log::info!("Reading new nginx log entry");
  //           let output = std::process::Command::new("tail")
  //             .args(["-n", "1", "/var/lib/nanocl/nginx/log/access.log"])
  //             .output()
  //             .expect("unable to get last nginx log entry.");
  //           let str = String::from_utf8(output.stdout).unwrap();
  //           let json_result = serde_json::from_str::<NginxLogPartial>(&str);
  //           match json_result {
  //             Err(err) => {
  //               log::error!("Parsed nginx log fail {:#?}", err);
  //             }
  //
  //             Ok(partial) => {
  //               repositories::nginx_log::create_log(partial, &pool).await;
  //             }
  //           }
  //         }
  //       }
  //       Ok(event) => log::warn!("Received broken event {:#?}", event),
  //       Err(e) => log::error!("Received error event {:#?}", e),
  //     }
  //   }
  // });

  log::info!("booted");
  // Return state
  Ok(BootState {
    pool: db_pool,
    docker_api: docker_api.to_owned(),
  })
}
