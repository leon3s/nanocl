use bollard::{
  Docker,
  errors::Error as DockerError,
  container::{CreateContainerOptions, Config},
  models::HostConfig,
};

use super::utils::*;

fn gen_nginx_host_conf() -> HostConfig {
  let binds = Some(vec![
    String::from(
      "/var/lib/nanocl/nginx/sites-enabled:/etc/nginx/sites-enabled",
    ),
    String::from("/var/lib/nanocl/nginx/log:/var/log/nginx"),
  ]);
  let network_mode = Some(String::from("host"));
  HostConfig {
    binds,
    network_mode,
    ..Default::default()
  }
}

async fn create_nginx_container(
  docker: &Docker,
  name: &str,
) -> Result<(), DockerError> {
  let image = Some("nanocl-proxy-nginx:latest");
  let labels = Some(gen_label_namespace("nanocl"));
  let host_config = Some(gen_nginx_host_conf());
  let options = Some(CreateContainerOptions { name });
  let config = Config {
    image,
    labels,
    host_config,
    tty: Some(true),
    attach_stdout: Some(true),
    attach_stderr: Some(true),
    ..Default::default()
  };
  docker.create_container(options, config).await?;
  Ok(())
}

pub async fn boot(docker: &Docker) -> Result<(), DockerError> {
  let container_name = "nanocl-proxy-nginx";
  build_service(docker, "nanocl-proxy-nginx").await?;
  let s_state = get_service_state(docker, container_name).await;

  if s_state == ServiceState::Uninstalled {
    create_nginx_container(docker, container_name).await?;
  }
  if s_state != ServiceState::Running {
    if let Err(err) = start_service(docker, container_name).await {
      eprintln!("error while starting {} {}", container_name, err);
    }
  }
  Ok(())
}
