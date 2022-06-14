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

  let pool = postgre::create_pool();

  let mut server = web::HttpServer::new(move || {
    web::App::new()
      // postgre pool
      .state(pool.clone())
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
      // TOTO remove it's for test websocket with javascript
      .service(
        fs::Files::new("/websocket", "./static/websocket")
          .index_file("index.html"),
      )
  });
  server = server.bind("0.0.0.0:8383")?;
  println!("starting server on http://0.0.0.0:8383");
  server.run().await
}
