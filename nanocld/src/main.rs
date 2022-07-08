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
use errors::DaemonError;

mod cli;
mod boot;
mod install;

mod utils;
mod errors;
mod server;
mod schema;
mod models;
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

  #[cfg(feature = "openapi")]
  {
    if args.genopenapi {
      let result = openapi::to_json();
      println!("{}", result);
      std::process::exit(0);
    }
  }
  let docker_api = match bollard::Docker::connect_with_unix(
    &args.docker_host,
    120,
    bollard::API_DEFAULT_VERSION,
  ) {
    Err(err) => {
      log::error!("{}", err);
      std::process::exit(1);
    }
    Ok(docker_api) => docker_api,
  };

  if args.install_services {
    if let Err(err) = install::install_services(&docker_api).await {
      match err {
        DaemonError::Docker(err) => match err {
          bollard::errors::Error::HyperResponseError { err } => {
            if err.is_connect() {
              log::error!(
                "unable to connect to docker host {}",
                &args.docker_host,
              );
              std::process::exit(1);
            }
            log::error!("{}", err);
            std::process::exit(1);
          }
          _ => {
            log::error!("{}", err);
            std::process::exit(1);
          }
        },
        _ => {
          log::error!("{}", err);
          std::process::exit(1);
        }
      }
    }
    return Ok(());
  }
  let state = match boot::boot(&docker_api).await {
    Err(err) => {
      log::error!("Error while trying to boot : {:?}", err);
      std::process::exit(1);
    }
    Ok(state) => state,
  };
  server::ntex::start_server(state).await?;
  log::info!("kill received exiting.");
  Ok(())
}
