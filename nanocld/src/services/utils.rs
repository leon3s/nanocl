use std::collections::HashMap;
use futures::StreamExt;
use bollard::{
  Docker,
  errors::Error as DockerError,
  image::{CreateImageOptions, BuildImageOptions},
  network::{CreateNetworkOptions, InspectNetworkOptions},
  container::StartContainerOptions,
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

/// # Generate labels width a namespace
///
/// # Arguments
/// - [namespace](str) the name of the namespace
///
/// # Return
/// [labels](HashMap) a hashmap of strings with namespace key as given value
///
/// # Examples
/// ```rust,norun
/// use crate::services;
///
/// services::utils::gen_labels_with_namespace("default");
/// ```
pub fn gen_labels_with_namespace(namespace: &str) -> HashMap<&str, &str> {
  let mut labels: HashMap<&str, &str> = HashMap::new();
  labels.insert("namespace", namespace);
  labels
}

/// # Start a service
/// Start service by it's name
///
/// # Arguments
/// - [docker](Docker) bollard docker instance
/// - [name](str) name of the service to start
///
/// # Return
/// if sucess return nothing a [docker error](DockerError) is returned if an error occur
///
/// # Examples
/// ```rust,norun
/// use crate::services;
///
/// services::utils::start_service(&docker, "nanocl-proxy-nginx").await;
/// ```
pub async fn start_service(
  docker: &Docker,
  name: &str,
) -> Result<(), DockerError> {
  docker
    .start_container(name, None::<StartContainerOptions<String>>)
    .await?;
  Ok(())
}

/// # Build a service
/// Build a nxthat service from github
///
/// # Arguments
/// - [docker](Docker) bollard docker instance
/// - [name](str) name of the service to build
///
/// # Return
/// if sucess return nothing a [docker error](DockerError) is returned if an error occur
///
/// /// # Examples
/// ```rust,norun
/// use crate::services;
///
/// services::utils::build_service(&docker, "nanocl-proxy-nginx").await;
/// ```
pub async fn build_service(
  docker: &Docker,
  service_name: &'static str,
) -> Result<(), DockerError> {
  let git_url = "https://github.com/nxthat/".to_owned();
  let image_url = git_url + service_name + ".git";
  let options = BuildImageOptions {
    dockerfile: "Dockerfile",
    t: service_name,
    remote: &image_url,
    ..Default::default()
  };
  let mut stream = docker.build_image(options, None, None);
  while let Some(output) = stream.next().await {
    if let Err(err) = output {
      return Err(err);
    }
  }
  Ok(())
}

/// # Install a service
/// Install a service from docker image
///
/// # Arguments
/// - [docker](Docker) bollard docker instance
/// - [name](str) name of the service to install
///
/// # Return
/// if sucess return nothing a [docker error](DockerError) is returned if an error occur
///
/// /// # Examples
/// ```rust,norun
/// use crate::services;
///
/// services::utils::install_service(&docker, "postgresql").await;
/// ```
pub async fn install_service(
  docker: &Docker,
  image_name: &'static str,
) -> Result<(), DockerError> {
  let mut stream = docker.create_image(
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
  }
  Ok(())
}

/// # Install a service
/// Install a service from docker image
///
/// # Arguments
/// - [docker](Docker) bollard docker instance
/// - [name](str) name of the service to install
///
/// # Return
/// /// if success return [network state](NetworkState)
/// a [docker error](DockerError) is returned if an error occur
///
/// /// # Examples
/// ```rust,norun
/// use crate::services;
///
/// services::utils::get_network_state(&docker, "network-name").await;
/// ```
pub async fn get_network_state(
  docker: &Docker,
  network_name: &str,
) -> Result<NetworkState, DockerError> {
  let config = InspectNetworkOptions {
    verbose: true,
    scope: "local",
  };

  let res = docker.inspect_network(network_name, Some(config)).await;
  if let Err(err) = res {
    match err {
      DockerError::DockerResponseServerError {
        status_code,
        message,
      } => {
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

/// # Create a network
/// Create a network by name with default settings using docker api
///
/// # Arguments
/// - [docker](Docker) bollard docker instance
/// - [name](str) name of the network to create
///
/// # Return
/// if sucess return nothing a [docker error](DockerError) is returned if an error occur
///
/// /// # Examples
/// ```rust,norun
/// use crate::services;
///
/// services::utils::create_network(&docker, "network-name").await;
/// ```
pub async fn create_network(
  docker: &Docker,
  network_name: &str,
) -> Result<(), DockerError> {
  let config = CreateNetworkOptions {
    name: network_name,
    ..Default::default()
  };
  docker.create_network(config).await?;
  Ok(())
}

/// # Get service state
/// Get state of a service by his name
///
/// # Arguments
/// - [docker](Docker) bollard docker instance
/// - [name](str) name of the service
///
/// # Return
/// if success return [service state](ServiceState)
/// a [docker error](DockerError) is returned if an error occur
///
/// # Examples
/// ```rust,norun
/// use crate::services;
///
/// services::utils::get_service_state(&docker, "nanocl-proxy-nginx").await;
/// ```
pub async fn get_service_state(
  docker: &Docker,
  container_name: &'static str,
) -> ServiceState {
  let resp = docker.inspect_container(container_name, None).await;
  if let Err(err) = resp {
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
