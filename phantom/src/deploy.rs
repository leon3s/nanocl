use bollard::{
  Docker,
  errors::Error as DockerError,
  container::{CreateContainerOptions, Config, StartContainerOptions},
};

use crate::docker_helper::install_service;

// todo lolz
pub async fn _test_deploy(
  docker: &Docker,
  _git_url: &'static str,
) -> Result<(), DockerError> {
  install_service(docker, "ubuntu:latest").await?;

  let options = Some(CreateContainerOptions { name: "nanoclqq" });
  let config = Config {
    image: Some("ubuntu:latest"),
    tty: Some(true),
    attach_stdout: Some(true),
    attach_stderr: Some(true),
    ..Default::default()
  };
  let result = match docker.create_container(options, config).await {
    Ok(result) => result,
    Err(err) => panic!("{:?}", err),
  };

  docker
    .start_container("nanoclqq", None::<StartContainerOptions<String>>)
    .await
    .unwrap();
  println!("{:?}", result);
  Ok(())
}
