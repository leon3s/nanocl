use bollard::{
  Docker,
  container::{
    CreateContainerOptions,
    Config
  },
  models::HostConfig,
};

use crate::docker_helper::*;

fn gen_nginx_host_conf() -> HostConfig {
  let binds = vec![
    String::from("/var/lib/nanocl/nginx/sites-enabled:/etc/nginx/sites-enabled"),
    String::from("/var/lib/nanocl/nginx/log:/var/log/nginx"),
  ];
  HostConfig {
    binds: Some(binds),
    network_mode: Some(String::from("host")),
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
      tty: Some(true),
      attach_stdout: Some(true),
      attach_stderr: Some(true),
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
  build_service(docker, "nanocl-proxy-nginx").await;
  let s_state = get_service_state(docker, container_name).await;

  println!("service state {:?}", s_state);
  if s_state == ServiceState::Uninstalled {
    create_nginx_container(docker, container_name).await;
  }
  if s_state != ServiceState::Running {
    if let Err(err) = start_service(docker, container_name).await {
      eprintln!("error while starting {} {}", container_name, err);
    }
  }
}
