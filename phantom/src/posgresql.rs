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
    StartContainerOptions
  }
};

use crate::docker_helper::*;

async fn create_postgre_container(docker: &Docker) {
  let options = Some(CreateContainerOptions{
    name: "nanoclq",
  });
  let mut port_bindings: HashMap<String, Option<Vec<PortBinding>>> = HashMap::new();
  port_bindings.insert(
    String::from("5432/tcp"),
    Some(vec![PortBinding {
      host_ip: Some(String::from("")),
      host_port: Some(String::from("5432")),
    }],
  ));
  let config = Config {
      image: Some("postgres"),
      env: Some(vec![
        "POSTGRES_USER=root",
        "POSTGRES_PASSWORD=root",
      ]),
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

async fn start_posgre_container(docker: &Docker) {
  docker.start_container(
    "nanoclq",
    None::<StartContainerOptions<String>>
  ).await.unwrap();
}

pub async fn init_posgre_container(docker: &Docker) {
  install_service(docker, "postgre:latest").await;
  let container_status = get_service_state(
    docker,
    "nanoclq",
  ).await;
  if container_status == ServiceState::Uninstalled {
    create_postgre_container(docker).await;
  }
  if container_status != ServiceState::Running {
    start_posgre_container(docker).await;
  }
  println!("im called");
}
