use std::collections::HashMap;

use futures::StreamExt;
use bollard::{Docker, image::CreateImageOptions, container::{CreateContainerOptions, Config, StartContainerOptions}, models::{HostConfig, PortBinding}};

#[derive(PartialEq)]
enum DepencencyStatus {
  Uninstalled,
  Running,
  Stopped,
}

async fn download_mongo_image(docker: &Docker) {
  let mut stream = docker
  .create_image(
      Some(CreateImageOptions {
          from_image: "mongo:latest",
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

async fn create_mongo_container(docker: &Docker) {
  let options = Some(CreateContainerOptions{
    name: "nanocldb",
  });
  let mut port_bindings: HashMap<String, Option<Vec<PortBinding>>> = HashMap::new();
  port_bindings.insert(
    String::from("27017/tcp"),
    Some(vec![PortBinding {
      host_ip: Some(String::from("")),
      host_port: Some(String::from("27017")),
    }],
  ));
  let config = Config {
      image: Some("mongo"),
      env: Some(vec![
        "MONGO_INITDB_ROOT_USERNAME=root",
        "MONGO_INITDB_ROOT_PASSWORD=root",
      ]),
      host_config: Some(HostConfig {
        port_bindings: Some(port_bindings),
        ..Default::default()
      }),
      ..Default::default()
  };
  let result = match docker.create_container(options, config).await {
    Ok(result) => result,
    Err(err) => panic!("{:?}", err),
  };
  println!("{:?}", result);
}

async fn get_dependency_status(docker: &Docker, container_name: &'static str) -> DepencencyStatus {
  let resp = docker.inspect_container(
    container_name,
    None
  ).await;
  if resp.is_err() {
    return DepencencyStatus::Uninstalled;
  }
  let body = resp.expect("ContainerInspectResponse");
  if let Some(state) = body.state {
    if let Some(running) = state.running {
      return if running {
        DepencencyStatus::Running
      } else {
        DepencencyStatus::Stopped
      };
    }
  }
  DepencencyStatus::Stopped
}

async fn start_mongo_container(docker: &Docker) {
  docker.start_container(
    "nanocldb",
    None::<StartContainerOptions<String>>
  ).await.unwrap();
}

pub async fn init_mongo_container(docker: &Docker) {
  download_mongo_image(docker).await;
  let container_status = get_dependency_status(
    docker,
    "nanocldb",
  ).await;
  if container_status == DepencencyStatus::Uninstalled {
    create_mongo_container(docker).await;
  }
  if container_status != DepencencyStatus::Running {
    start_mongo_container(docker).await;
  }
  println!("im called");
}

#[ntex::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
  println!("hello world! ");
  // let docker = Docker::unix("/var/run/docker.sock");
  // let pull_opts = PullOpts::builder().image("mongo").build();
  // let mut stream = docker.images().pull(&pull_opts);
  // while let Some(ouput) = stream.next().await {
  //   match ouput {
  //     Ok(ouput) => println!("{:?}", ouput),
  //     Err(err) => eprintln!("{:?}", err),
  //   }
  // }
  // println!("gg");
  let docker = Docker::connect_with_socket_defaults().unwrap();
  init_mongo_container(&docker).await;
  // let mut stream = docker
  // .create_image(
  //     Some(CreateImageOptions {
  //         from_image: "mongo:latest",
  //         ..Default::default()
  //     }),
  //     None,
  //     None,
  // );
  // while let Some(output) = stream.next().await {
  //   match output {
  //     Ok(o) => println!("{:?}", o),
  //     Err(err) => eprintln!("error : {}", err),
  //   }
  // }
  // .try_collect::<Vec<_>>()
  // .await?;
  // docker.create_image();
  Ok(())
}
