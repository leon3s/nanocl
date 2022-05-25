use std::collections::HashMap;
use bollard::{
  Docker,
  models::{
    PortBinding,
    HostConfig
  },
  container::{
    CreateContainerOptions,
    Config,
  }
};

use crate::docker_helper::*;

async fn create_postgre_container(docker: &Docker, name: &str) {
  let image = "postgres:latest";
  let env = vec![
    "POSTGRES_USER=root",
    "POSTGRES_PASSWORD=root",
  ];
  let labels = gen_namespace_label("nanocl");
  let mut port_bindings: HashMap<String, Option<Vec<PortBinding>>> = HashMap::new();
  port_bindings.insert(
    String::from("5432/tcp"),
    Some(vec![PortBinding {
      host_ip: None,
      host_port: Some(String::from("5432")),
    }],
  ));
  let options = Some(CreateContainerOptions{
    name,
  });
  let config = Config {
      image: Some(image),
      env: Some(env),
      labels: Some(labels),
      host_config: Some(HostConfig {
        port_bindings: Some(port_bindings),
        ..Default::default()
      }),
      ..Default::default()
  };
  let result = match docker.create_container(options, config).await {
    Ok(result) => result,
    Err(err) => panic!("{:?}", err),
  };
  println!("{:?}", result);
}

pub async fn ensure_start(docker: &Docker) {
  let container_name = "nanocl-db-postgre";
  install_service(docker, "postgres:latest").await;
  let container_status = get_service_state(
    docker,
    container_name,
  ).await;
  if container_status == ServiceState::Uninstalled {
    create_postgre_container(docker, container_name).await;
  }
  if container_status != ServiceState::Running {
    if let Err(err) = start_service(docker, container_name).await {
      eprintln!("error while starting {} {}", container_name, err);
    }
  }
  println!("im called");
}
