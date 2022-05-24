use std::{collections::HashMap, thread};
use futures::{StreamExt, TryStreamExt, FutureExt};
use bollard::{Docker, image::CreateImageOptions, container::{CreateContainerOptions, Config, StartContainerOptions, StatsOptions, Stats}, models::{HostConfig, PortBinding}};

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
          from_image: "postgres:latest",
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
    name: "nanoclq",
  });
  let mut port_bindings: HashMap<String, Option<Vec<PortBinding>>> = HashMap::new();
  port_bindings.insert(
    String::from("5432/tcp"),
    Some(vec![PortBinding {
      host_ip: Some(String::from("")),
      host_port: Some(String::from("5432")),
    }],
  ));
  let config = Config {
      image: Some("postgres"),
      env: Some(vec![
        "POSTGRES_USER=root",
        "POSTGRES_PASSWORD=root",
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
    "nanoclq",
    None::<StartContainerOptions<String>>
  ).await.unwrap();
}

pub async fn init_mongo_container(docker: &Docker) {
  download_mongo_image(docker).await;
  let container_status = get_dependency_status(
    docker,
    "nanoclq",
  ).await;
  if container_status == DepencencyStatus::Uninstalled {
    create_mongo_container(docker).await;
  }
  if container_status != DepencencyStatus::Running {
    start_mongo_container(docker).await;
  }
  println!("im called");
}

pub async fn test_deploy(docker: &Docker, git_url: &'static str) {
  let mut stream = docker
  .create_image(
      Some(CreateImageOptions {
          from_image: "ubuntu:latest",
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

  let options = Some(CreateContainerOptions {
    name: "nanoclqq",
  });
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

  docker.start_container(
    "nanoclqq",
    None::<StartContainerOptions<String>>
  )
  .await
  .unwrap();
  println!("{:?}", result);
}

type Callback = fn (stats: Stats);

async fn test_stats(docker: &Docker, callback: Callback) {
  let options = Some(StatsOptions {
    stream: false,
    one_shot: false,
  });
  let mut stream = docker.stats("nanoclq", options);
  let stats = stream.try_next().await;
  match stats {
    Ok(stats) => {
      match stats {
        Some(stats) => {
          println!("{:?}", stats);
          callback(stats);
        },
        None => {
          eprintln!("Stats are empty");
        }
      }
    },
    Err(err) => {
      eprintln!("error while collecting stats {}", err);
    }
  }
  println!("finished .");
}

#[ntex::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
  let docker = Docker::connect_with_socket_defaults().unwrap();
  test_stats(&docker, |_| {
    println!("sucess");
  }).await;
  // test_deploy(&docker, "https://github.com/leon3s/express-test-deploy").await;
  // let addrs = nix::ifaddrs::getifaddrs().unwrap();
  // for ifaddr in addrs {
  //   println!("[{}]", ifaddr.interface_name);
  //   match ifaddr.address {
  //     Some(address) => {
  //       println!("address {}", address);
  //     },
  //     None => {
  //       eprintln!("interface {} with unsupported address family",
  //                ifaddr.interface_name);
  //     }
  //   }
  //   match ifaddr.broadcast {
  //     Some(broadcast) => {
  //       println!("{:?}", broadcast.to_string());
  //     },
  //     None => {

  //     }
  //   }
  //   match ifaddr.netmask {
  //     Some(netmask) => {
  //       println!("{}", netmask.to_string());
  //     }
  //     None => {
  //     }
  //   }
  //   println!("-------------");
  // }
  // println!("hello world! ");
  Ok(())
}
