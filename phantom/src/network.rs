use bollard::{
  Docker,
  errors::Error as DockerError,
};

use crate::docker_helper;

pub async fn ensure_start(docker: &Docker) -> Result<(), DockerError> {
  let network_name = "nanocl";
  let state = docker_helper::get_network_state(docker, network_name).await?;
  if state == docker_helper::NetworkState::NotFound {
    docker_helper::create_network(docker, network_name).await?;
  }
  Ok(())
}
