use bollard::{
  Docker,
  models::HostConfig,
  errors::Error as DockerError,
  container::{CreateContainerOptions, Config},
  exec::{CreateExecOptions, StartExecOptions},
};

use super::utils::*;

pub async fn reload_config(docker: &Docker) -> Result<(), DockerError> {
  let container_name = "nanocl-proxy-nginx";
  let config = CreateExecOptions {
    cmd: Some(vec!["nginx", "-s", "reload"]),
    attach_stdout: Some(true),
    attach_stderr: Some(true),
    ..Default::default()
  };
  let res = docker.create_exec(container_name, config).await?;
  let config = StartExecOptions { detach: false };
  docker.start_exec(&res.id, Some(config)).await?;
  Ok(())
}

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
  let labels = Some(gen_labels_with_namespace("nanocl"));
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
  let s_state = get_service_state(docker, container_name).await;
  if s_state == ServiceState::Uninstalled {
    create_nginx_container(docker, container_name).await?;
  }
  if s_state != ServiceState::Running {
    if let Err(err) = start_service(docker, container_name).await {
      log::error!("error while starting {} {}", container_name, err);
    }
  }
  Ok(())
}
