use ntex::web;

use crate::openapi;
use crate::controllers;
use crate::boot::DaemonState;

pub async fn start_server(state: DaemonState) -> std::io::Result<()> {
  let mut server = web::HttpServer::new(move || {
    web::App::new()
      // bind postgre pool to state
      .state(state.pool.clone())
      // bind docker connection to state
      .state(state.docker_api.clone())
      // bind docker api
      // Default logger middleware
      .wrap(web::middleware::Logger::default())
      // Set Json body max size
      .app_state(web::types::JsonConfig::default().limit(4096))
      // bind /explorer
      .configure(openapi::ntex_config)
      // bind controller namespace
      .configure(controllers::namespace::ntex_config)
      // bind controller git repository
      .configure(controllers::git_repository::ntex_config)
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
  log::info!("binding on /run/nanocl/nanocl.sock");
  server = server.bind_uds("/run/nanocl/nanocl.sock")?;
  #[cfg(debug_assertions)]
  {
    log::info!("binding on http://0.0.0.0:8383");
    server = server.bind("0.0.0.0:8383")?;
  }
  log::info!("daemon started");
  server.run().await
}
