use ntex::web;
use ntex_files as fs;

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
    // TOTO remove it's for test websocket with javascript
    // .service(
    //   fs::Files::new("/websocket", "./static/websocket")
    //     .index_file("index.html"),
    // )
  });
  server = server.bind_uds("/run/nanocl/nanocl.sock")?;
  server = server.bind("0.0.0.0:8383")?;
  log::info!("http started on http://0.0.0.0:8383");
  server.run().await
}
