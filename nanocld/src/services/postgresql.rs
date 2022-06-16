use std::collections::HashMap;
use bollard::{
  Docker,
  errors::Error as DockerError,
  models::{PortBinding, HostConfig},
  container::{CreateContainerOptions, Config},
};

use super::utils::*;

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
      eprintln!("error while starting {} {}", container_name, err);
    }
  }
  Ok(())
}
