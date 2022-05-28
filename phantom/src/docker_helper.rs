use std::collections::HashMap;
use futures::StreamExt;
use bollard::{
  Docker,
  image::{
    CreateImageOptions,
    BuildImageOptions,
  },
  errors::Error as DockerError,
  network::ConnectNetworkOptions,
  container::StartContainerOptions,
};

#[derive(Debug, PartialEq)]
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

pub async fn build_service(docker: &Docker, service_name: &'static str) {
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
    match output {
      Err(err) => panic!("{:?}", err),
      Ok(output) => println!("{:?}", output),
    }
  }
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
      Err(err) => panic!("{:?}", err),
      Ok(output) => println!("{:?}", output),
    }
  }
}

pub async fn connect_to_network(docker: &Docker, container_name: &str, network_name: &str) {
  let config = ConnectNetworkOptions {
    container: container_name,
    ..Default::default()
  };
  let resp = docker.connect_network(network_name, config).await;
  match resp {
    Err(err) => panic!("{:?}", err),
    Ok(body) => println!("{:?}", body),
  }
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
