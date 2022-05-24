use std::collections::HashMap;

use bollard::{Docker, container::{CreateContainerOptions, Config, StartContainerOptions}, models::{PortBinding, HostConfig}};

use crate::docker_helper::*;

async fn create_nginx_container(docker: &Docker) {
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

async fn start_nginx_service(docker: &Docker) {
  docker.start_container(
    "nanoclq",
    None::<StartContainerOptions<String>>
  ).await.unwrap();
}

pub async fn ctrl_nginx(docker: &Docker) {
  install_service(docker, "nginx:latest").await;
  let s_state = get_service_state(docker, "nanocl-ctrl-nginx").await;

  if s_state == ServiceState::Uninstalled {
    create_nginx_container(docker).await;
  }
  if s_state != ServiceState::Running {
  }
}
