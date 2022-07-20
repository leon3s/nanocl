use std::path::Path;

use ntex::web;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;

use ntex::http::StatusCode;
use bollard::{
  Docker,
  models::HostConfig,
  errors::Error as DockerError,
  container::{CreateContainerOptions, Config},
};

use crate::{
  models::{Pool, DBConn},
  config::DaemonConfig,
};
use crate::errors::HttpResponseError;

use super::utils::*;

fn gen_postgre_host_conf(config: &DaemonConfig) -> HostConfig {
  let path = Path::new(&config.state_dir).join("postgresql/data");

  let binds = vec![format!("{}:/var/lib/postgresql/data", path.display())];

  HostConfig {
    binds: Some(binds),
    network_mode: Some(String::from("nanoclservices0")),
    ..Default::default()
  }
}

async fn create_postgre_container(
  name: &str,
  config: &DaemonConfig,
  docker_api: &Docker,
) -> Result<(), DockerError> {
  let image = Some("postgres:latest");
  let env = Some(vec![
    "POSTGRES_USER=root",
    "POSTGRES_PASSWORD=root",
    "POSTGRES_DB=nanocl",
  ]);
  let labels = Some(gen_labels_with_namespace("nanocl"));
  let host_config = Some(gen_postgre_host_conf(config));
  let options = Some(CreateContainerOptions { name });
  let config = Config {
    image,
    env,
    labels,
    host_config,
    hostname: Some(name),
    domainname: Some(name),
    ..Default::default()
  };
  docker_api.create_container(options, config).await?;
  Ok(())
}

/// # Create pool
/// Create an pool connection to postgres database
///
/// # Returns
/// - [Pool](Pool) R2d2 pool connection for postgres
///
/// # Examples
/// ```
/// let pool = create_pool();
/// ```
pub fn create_pool(host: String) -> Pool {
  let db_url = "postgres://root:root@".to_owned() + &host + "/nanocl";
  let manager = ConnectionManager::<PgConnection>::new(db_url);
  r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create pool.")
}

/// # Get connection from a pool
///
/// # Arguments
/// [pool](web::types::State<Pool>) a pool wrapped in ntex State
///
pub fn get_pool_conn(
  pool: &web::types::State<Pool>,
) -> Result<DBConn, HttpResponseError> {
  let conn = match pool.get() {
    Ok(conn) => conn,
    Err(_) => {
      return Err(HttpResponseError {
        msg: String::from("unable to connect to nanocl-db"),
        status: StatusCode::INTERNAL_SERVER_ERROR,
      });
    }
  };
  Ok(conn)
}

pub async fn get_postgres_ip(
  docker_api: &Docker,
) -> Result<String, HttpResponseError> {
  let container = docker_api
    .inspect_container("nanocl-db-postgre", None)
    .await?;

  let networks = container
    .network_settings
    .ok_or(HttpResponseError {
      msg: String::from("unable to get nanocl-db-postgre network nettings"),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    })?
    .networks
    .ok_or(HttpResponseError {
      msg: String::from("unable to get nanocl-db-postgre networks"),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

  let ip_address = networks
    .get("nanoclservices0")
    .ok_or(HttpResponseError {
      msg: String::from("unable to get nanocl-db-postgre network nanocl"),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    })?
    .ip_address
    .as_ref()
    .ok_or(HttpResponseError {
      msg: String::from("unable to get nanocl-db-postgre network nanocl"),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

  Ok(ip_address.to_owned())
}

pub async fn boot(
  config: &DaemonConfig,
  docker_api: &Docker,
) -> Result<(), DockerError> {
  let container_name = "nanocl-db-postgre";
  let s_state = get_service_state(container_name, docker_api).await;

  if s_state == ServiceState::Uninstalled {
    create_postgre_container(container_name, config, docker_api).await?;
  }
  if s_state != ServiceState::Running {
    if let Err(err) = start_service(container_name, docker_api).await {
      log::error!("error while starting {} {}", container_name, err);
    }
  }
  Ok(())
}
