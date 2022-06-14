//! nanocl daemon
//!
//! Provides an api to manage clusters network and containers
//! there are these advantages:
//! - Opensource
//! - [`Easy`]
//!
#[macro_use]
extern crate diesel;

use ntex::web;
use bollard::Docker;
use ntex_files as fs;

mod utils;
mod docker;
mod models;
mod schema;
mod openapi;
mod postgre;
mod services;
mod controllers;
mod repositories;

/// nanocld is the daemon to manager namespace cluster network and cargos
///
/// # Example
/// ```sh
/// nanocld --version
/// ```
#[ntex::main]
async fn main() -> std::io::Result<()> {
  env_logger::init();

  let pool = postgre::create_pool();

  let docker = Docker::connect_with_socket_defaults().unwrap();

  let mut server = web::HttpServer::new(move || {
    web::App::new()
      .state(docker.clone())
      .state(pool.clone())
      .wrap(web::middleware::Logger::default())
      .app_state(web::types::JsonConfig::default().limit(4096))
      .configure(openapi::ntex_config)
      .configure(controllers::namespace::ntex_config)
      .configure(controllers::cluster_network::ntex_config)
      .configure(controllers::git_repository::ntex_config)
      .configure(controllers::cluster::ntex_config)
      .service(
        fs::Files::new("/websocket", "./static/websocket")
          .index_file("index.html"),
      )
  });
  server = server.bind("0.0.0.0:8383")?;
  println!("starting server on http://0.0.0.0:8383");
  server.run().await
}
