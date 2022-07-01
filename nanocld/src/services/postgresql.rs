use std::collections::HashMap;
use bollard::{
  Docker,
  errors::Error as DockerError,
  models::{PortBinding, HostConfig},
  container::{CreateContainerOptions, Config},
};
use ntex::http::StatusCode;

use crate::controllers::errors::HttpError;

use super::{utils::*, errors::docker_error};

fn gen_postgre_host_conf() -> HostConfig {
  let mut port_bindings: HashMap<String, Option<Vec<PortBinding>>> =
    HashMap::new();
  port_bindings.insert(
    String::from("5432/tcp"),
    Some(vec![PortBinding {
      host_ip: None,
      host_port: Some(String::from("5432")),
    }]),
  );

  let binds = vec![
    String::from(
      "/var/lib/nanocl/nginx/sites-enabled:/etc/nginx/sites-enabled",
    ),
    String::from("/var/lib/nanocl/postgre/data:/var/lib/postgresql/data"),
  ];

  HostConfig {
    binds: Some(binds),
    port_bindings: Some(port_bindings),
    network_mode: Some(String::from("nanocl")),
    ..Default::default()
  }
}

async fn create_postgre_container(
  docker: &Docker,
  name: &str,
) -> Result<(), DockerError> {
  let image = Some("postgres:latest");
  let env = Some(vec!["POSTGRES_USER=root", "POSTGRES_PASSWORD=root"]);
  let labels = Some(gen_labels_with_namespace("nanocl"));
  let host_config = Some(gen_postgre_host_conf());
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
  docker.create_container(options, config).await?;
  Ok(())
}

pub async fn boot(docker: &Docker) -> Result<(), DockerError> {
  let container_name = "nanocl-db-postgre";
  install_service(docker, "postgres:latest").await?;
  let s_state = get_service_state(docker, container_name).await;

  if s_state == ServiceState::Uninstalled {
    create_postgre_container(docker, container_name).await?;
  }
  if s_state != ServiceState::Running {
    if let Err(err) = start_service(docker, container_name).await {
      log::error!("error while starting {} {}", container_name, err);
    }
  }
  Ok(())
}

pub async fn get_postgres_ip(docker: &Docker) -> Result<String, HttpError> {
  let container = docker
    .inspect_container("nanocl-db-postgre", None)
    .await
    .map_err(docker_error)?;

  let networks = container
    .network_settings
    .ok_or(HttpError {
      msg: String::from("unable to get nanocl-db-postgre network nettings"),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    })?
    .networks
    .ok_or(HttpError {
      msg: String::from("unable to get nanocl-db-postgre networks"),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

  let ip_address = networks
    .get("nanocl")
    .ok_or(HttpError {
      msg: String::from("unable to get nanocl-db-postgre network nanocl"),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    })?
    .ip_address
    .as_ref()
    .ok_or(HttpError {
      msg: String::from("unable to get nanocl-db-postgre network nanocl"),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

  Ok(ip_address.to_owned())
}
