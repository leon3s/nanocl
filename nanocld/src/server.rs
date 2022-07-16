use ntex::web;
use futures::channel::mpsc::UnboundedSender;

use crate::openapi;
use crate::controllers;
use crate::boot::BootState;
use crate::config::DaemonConfig;
use crate::events::system::EventMessage;

pub async fn start<'a>(
  config: DaemonConfig,
  event_system: UnboundedSender<EventMessage>,
  boot_state: BootState,
) -> std::io::Result<()> {
  let hosts = config.hosts.to_owned();
  let mut server = web::HttpServer::new(move || {
    web::App::new()
      // bind event system
      .state(event_system.clone())
      // bind config state
      .state(config.clone())
      // bind postgre pool to state
      .state(boot_state.pool.clone())
      // bind docker api
      .state(boot_state.docker_api.clone())
      // Default logger middleware
      .wrap(web::middleware::Logger::default())
      // Set Json body max size
      .app_state(web::types::JsonConfig::default().limit(4096))
      // bind /explorer
      .configure(openapi::ntex_config)
      // bind controller system
      .configure(controllers::system::ntex_config)
      // bind controller namespace
      .configure(controllers::namespace::ntex_config)
      // bind controller git repository
      .configure(controllers::git_repository::ntex_config)
      // bind nginx log
      .configure(controllers::nginx_log::ntex_config)
      // bind controller container_image
      .configure(controllers::container_image::ntex_config)
      // bind controller cluster
      .configure(controllers::cluster::ntex_config)
      // bind controller cluster variables
      .configure(controllers::cluster_variable::ntex_config)
      // bind controller cluster network
      .configure(controllers::cluster_network::ntex_config)
      // bind controller nginx template
      .configure(controllers::nginx_template::ntex_config)
      // bind controller cargo
      .configure(controllers::cargo::ntex_config)
  });
  let mut count = 0;
  let len = hosts.len();
  while count < len {
    let host = &hosts[count];
    if host.starts_with("unix://") {
      let addr = host.replace("unix://", "");
      server = server.bind_uds(&addr)?;
      log::info!("listening on {}", &host);
    } else if host.starts_with("tcp://") {
      let addr = host.replace("tcp://", "");
      server = server.bind(&addr)?;
      log::info!("listening on {}", &host);
    } else {
      log::warn!("{} is not valid use tcp:// or unix:// as protocol", host);
    }
    count += 1;
  }
  #[cfg(debug_assertions)]
  {
    server = server.bind("0.0.0.0:8383")?;
    log::info!("listening on http://0.0.0.0:8383");
  }
  log::info!("http server started");
  server.run().await
}
