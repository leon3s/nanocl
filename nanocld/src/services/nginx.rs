use std::path::Path;

use bollard::{
  Docker,
  models::HostConfig,
  errors::Error as DockerError,
  container::{CreateContainerOptions, Config},
  exec::{CreateExecOptions, StartExecOptions},
};

use crate::config::DaemonConfig;

use super::utils::*;

pub async fn reload_config(docker_api: &Docker) -> Result<(), DockerError> {
  let container_name = "nanocl-proxy-nginx";
  let config = CreateExecOptions {
    cmd: Some(vec!["nginx", "-s", "reload"]),
    attach_stdout: Some(true),
    attach_stderr: Some(true),
    ..Default::default()
  };
  let res = docker_api.create_exec(container_name, config).await?;
  let config = StartExecOptions { detach: false };
  docker_api.start_exec(&res.id, Some(config)).await?;
  Ok(())
}

fn gen_nginx_host_conf(config: &DaemonConfig) -> HostConfig {
  let sites_path = Path::new(&config.state_dir).join("nginx/sites-enabled");
  let log_path = Path::new(&config.state_dir).join("nginx/log");
  let ssl_path = Path::new(&config.state_dir).join("nginx/ssl");
  let letsencrypt_path = Path::new(&config.state_dir).join("nginx/letsencrypt");
  let binds = Some(vec![
    format!("{}:/etc/nginx/sites-enabled", sites_path.display()),
    format!("{}:/var/log/nginx", log_path.display()),
    format!("{}:/etc/nginx/ssl", ssl_path.display()),
    format!("{}:/etc/letsencrypt", letsencrypt_path.display()),
  ]);
  let network_mode = Some(String::from("host"));
  HostConfig {
    binds,
    network_mode,
    ..Default::default()
  }
}

async fn create_nginx_container(
  name: &str,
  config: &DaemonConfig,
  docker_api: &Docker,
) -> Result<(), DockerError> {
  let image = Some("nanocl-proxy-nginx:latest");
  let labels = Some(gen_labels_with_namespace("nanocl"));
  let host_config = Some(gen_nginx_host_conf(config));
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
  docker_api.create_container(options, config).await?;
  Ok(())
}

pub async fn boot(
  config: &DaemonConfig,
  docker_api: &Docker,
) -> Result<(), DockerError> {
  let container_name = "nanocl-proxy-nginx";
  let s_state = get_service_state(container_name, docker_api).await;
  if s_state == ServiceState::Uninstalled {
    create_nginx_container(container_name, config, docker_api).await?;
  }
  if s_state != ServiceState::Running {
    if let Err(err) = start_service(container_name, docker_api).await {
      log::error!("error while starting {} {}", container_name, err);
    }
  }
  Ok(())
}
