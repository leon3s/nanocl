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
use ntex_files as fs;

mod boot;
mod utils;
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

  let state = match boot::boot().await {
    Err(err) => panic!("daemon boot fail {:?}", err),
    Ok(state) => state,
  };
  let mut server = web::HttpServer::new(move || {
    web::App::new()
      // bind postgre pool to state
      .state(state.pool.clone())
      // bind docker connection to state
      .state(state.docker.clone())
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
      // bind controller cluster network
      .configure(controllers::cluster_network::ntex_config)
      // bind controller cargo
      .configure(controllers::cargo::ntex_config)
      // TOTO remove it's for test websocket with javascript
      .service(
        fs::Files::new("/websocket", "./static/websocket")
          .index_file("index.html"),
      )
  });
  server = server.bind("0.0.0.0:8383")?;
  println!("running server on http://0.0.0.0:8383");
  server.run().await?;
  println!("exiting");
  Ok(())
}
