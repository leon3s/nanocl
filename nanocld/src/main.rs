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

use clap::Parser;

mod cli;
mod boot;
mod utils;
mod server;
mod models;
mod schema;
mod openapi;
mod services;
mod controllers;
mod repositories;

/// nanocld is the daemon to manager namespace cluster network and cargoes
///
/// # Example
/// ```sh
/// nanocld --version
/// ```
#[ntex::main]
async fn main() -> std::io::Result<()> {
  let args = cli::Cli::parse();
  // building env logger
  if std::env::var("LOG_LEVEL").is_err() {
    std::env::set_var("LOG_LEVEL", "nanocld=info,warn,error,nanocld=debug");
  }
  env_logger::Builder::new().parse_env("LOG_LEVEL").init();

  log::info!("daemon booting");
  let state = match boot::boot().await {
    Err(err) => {
      log::error!("Error while trying to boot : {:?}", err);
      std::process::exit(1);
    }
    Ok(state) => state,
  };
  log::info!("daemon booted");
  if args.boot_only {
    return Ok(());
  }
  log::info!("daemon starting");
  server::ntex::start_server(state).await?;
  log::info!("kill received exiting.");
  Ok(())
}
