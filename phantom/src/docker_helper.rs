use std::collections::HashMap;
use futures::StreamExt;
use bollard::{
  Docker,
  errors::Error as DockerError,
  image::CreateImageOptions,
  container::StartContainerOptions,
};

#[derive(PartialEq)]
pub enum ServiceState {
  Uninstalled,
  Running,
  Stopped,
}

pub fn gen_namespace_label(namespace: &str) -> HashMap<&str, &str> {
  let mut labels: HashMap<&str, &str> = HashMap::new();
  labels.insert("namespace", namespace);
  labels
}

pub async fn start_service(docker: &Docker, name: &str) -> Result<(), DockerError> {
  docker.start_container(
    name,
    None::<StartContainerOptions<String>>
  ).await?;
  Ok(())
}

pub async fn install_service(docker: &Docker, image_name: &'static str) {
  let mut stream = docker
  .create_image(
      Some(CreateImageOptions {
          from_image: image_name,
          ..Default::default()
      }),
      None,
      None,
  );
  while let Some(output) = stream.next().await {
    match output {
      Ok(output) => println!("{:?}", output),
      Err(err) => panic!("{:?}", err),
    }
  }
}

pub async fn get_service_state(docker: &Docker, container_name: &'static str) -> ServiceState {
  let resp = docker.inspect_container(
    container_name,
    None
  ).await;
  if resp.is_err() {
    return ServiceState::Uninstalled;
  }
  let body = resp.expect("ContainerInspectResponse");
  if let Some(state) = body.state {
    if let Some(running) = state.running {
      return if running {
        ServiceState::Running
      } else {
        ServiceState::Stopped
      };
    }
  }
  ServiceState::Stopped
}
