//! nanocl daemon
//!
//! Provides an api to manage clusters network and containers
//! there are these advantages:
//! - Opensource
//! - [`Easy`]
//!
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

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
  // building env logger
  if std::env::var("LOG_LEVEL").is_err() {
    std::env::set_var("LOG_LEVEL", "nanocld=info,warn,error");
  }
  env_logger::Builder::new().parse_env("LOG_LEVEL").init();

  log::info!("booting...");
  let state = match boot::boot().await {
    Err(err) => {
      log::error!("Error while trying to boot : {:?}", err);
      std::process::exit(1);
    }
    Ok(state) => state,
  };
  log::info!("booted");
  log::info!("starting http");
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
      // bind controller cluster network
      .configure(controllers::cluster_network::ntex_config)
      // bind controller nginx template
      .configure(controllers::nginx_template::ntex_config)
      // bind controller cargo
      .configure(controllers::cargo::ntex_config)
      // TOTO remove it's for test websocket with javascript
      .service(
        fs::Files::new("/websocket", "./static/websocket")
          .index_file("index.html"),
      )
  });
  server = server.bind_uds("/run/nanocl/nanocl.sock")?;
  server = server.bind("0.0.0.0:8383")?;
  log::info!("http started on http://0.0.0.0:8383");
  server.run().await?;
  log::info!("kill received existing.");
  Ok(())
}
