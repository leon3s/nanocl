use std::collections::HashMap;

use bollard::{
  Docker,
  container::{
    CreateContainerOptions,
    Config
  },
  models::{
    PortBinding,
    HostConfig
  }
};

use crate::docker_helper::*;

fn gen_nginx_host_conf() -> HostConfig {
  let mut port_bindings: HashMap<String, Option<Vec<PortBinding>>> = HashMap::new();
  port_bindings.insert(
    String::from("80/tcp"),
    Some(vec![PortBinding {
      host_ip: None,
      host_port: Some(String::from("80")),
    }],
  ));
  let binds = vec![
    String::from("/var/lib/nanocl/nginx/sites-enabled:/etc/nginx/sites-enabled"),
  ];
  HostConfig {
    binds: Some(binds),
    port_bindings: Some(port_bindings),
    ..Default::default()
  }
}

async fn create_nginx_container(docker: &Docker, name: &str) {
  let image = Some("nginx:latest");
  let labels = Some(gen_namespace_label("nanocl"));
  let host_config = Some(gen_nginx_host_conf());
  let options = Some(CreateContainerOptions{
    name,
  });
  let config = Config {
      image,
      labels,
      host_config,
      ..Default::default()
  };
  let result = match docker.create_container(options, config).await {
    Err(err) => panic!("{:?}", err),
    Ok(result) => result,
  };
  println!("{:?}", result);
}

pub async fn ensure_start(docker: &Docker) {
  let container_name = "nanocl-proxy-nginx";
  install_service(docker, "nginx:latest").await;
  let s_state = get_service_state(docker, "nanocl-ctrl-nginx").await;

  if s_state == ServiceState::Uninstalled {
    create_nginx_container(docker, container_name).await;
  }
  if s_state != ServiceState::Running {
    if let Err(err) = start_service(docker, container_name).await {
      eprintln!("error while starting {} {}", container_name, err);
    }
  }
}
