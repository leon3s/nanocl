use std::collections::HashMap;
use futures::StreamExt;
use bollard::{
  Docker,
  errors::Error as DockerError,
  image::{
    CreateImageOptions,
    BuildImageOptions,
  },
  network::{
    ConnectNetworkOptions,
    CreateNetworkOptions, InspectNetworkOptions,
  },
  container::StartContainerOptions, models::Network,
};

#[derive(Debug, PartialEq)]
pub enum ServiceState {
  Uninstalled,
  Running,
  Stopped,
}

#[derive(Debug, PartialEq)]
pub enum NetworkState {
  NotFound,
  Ready,
}

pub fn gen_label_namespace(namespace: &str) -> HashMap<&str, &str> {
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

pub async fn build_service(docker: &Docker, service_name: &'static str) -> Result<(), DockerError>{
  let git_url = "https://github.com/nxthat/".to_owned();
  let image_url = git_url + service_name + ".git";
  let options = BuildImageOptions{
    dockerfile: "Dockerfile",
    t: service_name,
    remote: &image_url,
    ..Default::default()
  };
  let mut stream = docker.build_image(
    options,
    None,
    None,
  );
  while let Some(output) = stream.next().await {
    if let Err(err) = output {
      return Err(err);
    }
  };
  Ok(())
}

pub async fn install_service(docker: &Docker, image_name: &'static str) -> Result<(), DockerError> {
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
    if let Err(err) = output {
      return Err(err);
    }
  };
  Ok(())
}

pub async fn connect_to_network(docker: &Docker, container_name: &str, network_name: &str) -> Result<(), DockerError> {
  let config = ConnectNetworkOptions {
    container: container_name,
    ..Default::default()
  };
  docker.connect_network(network_name, config).await?;
  Ok(())
}

pub async fn get_network_state(docker: &Docker, network_name: &str) -> Result<NetworkState, DockerError> {
  let config = InspectNetworkOptions {
    verbose: true,
    scope: "local"
  };

  let res = docker.inspect_network(network_name, Some(config)).await;
  if let Err(err) = res {
    match err {
      DockerError::DockerResponseServerError { status_code, message } => {
        if status_code == 404 {
          return Ok(NetworkState::NotFound);
        }
        return Err(DockerError::DockerResponseServerError {
          status_code,
          message,
        });
      }
      _ => return Err(err),
    }
  }
  Ok(NetworkState::Ready)
}

pub async fn create_network(docker: &Docker, network_name: &str) -> Result<(), DockerError> {
  let config = CreateNetworkOptions {
    name: network_name,
    ..Default::default()
  };
  docker.create_network(config).await?;
  Ok(())
}

pub async fn get_service_state(docker: &Docker, container_name: &'static str) -> ServiceState {
  let resp = docker.inspect_container(
    container_name,
    None
  ).await;
  if let Err(err) = resp {
    println!("error : {:?}", err);
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
