use ntex::web;
use ntex::http::StatusCode;
use std::collections::HashMap;
use futures::StreamExt;
use futures::stream::FuturesUnordered;
use crate::models::{Pool, CargoItem, CargoPortItem, ClusterItem};

use crate::repositories::cargo_port;
use crate::controllers::errors::HttpError;

use crate::utils::get_free_port;

pub async fn start_cargo(
  item: CargoItem,
  docker_api: &web::types::State<bollard::Docker>,
  pool: &web::types::State<Pool>,
) -> Result<Vec<String>, HttpError> {
  let mut container_names: Vec<String> = Vec::new();
  let image_name = item.image_name.clone();
  let container_name =
    item.key.to_owned() + "-" + &image_name.replace(':', "-");

  container_names.push(container_name.to_owned());
  let ports = cargo_port::list_for_cargo(item.to_owned(), pool).await?;

  log::debug!("item found {:?}", item);
  log::debug!("image name not empty {:?}", image_name.clone());
  if docker_api.inspect_image(&item.image_name).await.is_err() {
    return Err(HttpError {
      msg: String::from("image name is not valid"),
      status: StatusCode::BAD_REQUEST,
    });
  }
  let image = Some(image_name.clone());
  let options = Some(bollard::container::CreateContainerOptions {
    name: container_name.clone(),
  });
  let mut port_bindings: HashMap<
    String,
    Option<Vec<bollard::models::PortBinding>>,
  > = HashMap::new();
  let updated_ports = ports
    .into_iter()
    .map(|port| -> Result<CargoPortItem, HttpError> {
      let new_port = get_free_port()?;
      port_bindings.insert(
        port.to.to_string() + "/tcp",
        Some(vec![bollard::models::PortBinding {
          host_ip: None,
          host_port: Some(new_port.to_string()),
        }]),
      );
      let item = CargoPortItem {
        key: port.key,
        cargo_key: port.cargo_key,
        to: port.to,
        from: new_port as i32,
      };
      Ok(item)
    })
    .collect::<Result<Vec<CargoPortItem>, HttpError>>()?;

  let mut labels: HashMap<String, String> = HashMap::new();
  labels.insert(String::from("namespace"), item.namespace_name.to_owned());
  labels.insert(String::from("cargo"), item.key.to_owned());
  cargo_port::update_many(updated_ports, pool).await?;
  let config = bollard::container::Config {
    image,
    tty: Some(true),
    labels: Some(labels),
    attach_stdout: Some(true),
    attach_stderr: Some(true),
    host_config: Some(bollard::models::HostConfig {
      port_bindings: Some(port_bindings),
      ..Default::default()
    }),
    ..Default::default()
  };
  if let Err(err) = docker_api.create_container(options, config).await {
    return match err {
      bollard::errors::Error::DockerResponseServerError {
        message,
        status_code,
      } => Err(HttpError {
        msg: message,
        status: StatusCode::from_u16(status_code).unwrap(),
      }),
      _ => Err(HttpError {
        msg: format!("unable to create container {:?}", err),
        status: StatusCode::BAD_REQUEST,
      }),
    };
  }

  if let Err(err) = docker_api
    .start_container(
      &container_name,
      None::<bollard::container::StartContainerOptions<String>>,
    )
    .await
  {
    return Err(HttpError {
      msg: format!("unable to start container {:?}", err),
      status: StatusCode::BAD_REQUEST,
    });
  }

  Ok(container_names)
}

pub async fn start_cargo_in_cluster(
  item: CargoItem,
  cluster: ClusterItem,
  docker_api: &web::types::State<bollard::Docker>,
  pool: &web::types::State<Pool>,
) -> Result<(), HttpError> {
  let key = &cluster.key;
  let network_name = item.network_name.to_owned();
  let container_names = start_cargo(item, docker_api, pool).await?;

  if let Some(network_name) = &network_name {
    let vec_futures = container_names
      .into_iter()
      .map(|container_name| async move {
        let network_name = key.to_owned() + "-" + network_name;
        let config = bollard::network::ConnectNetworkOptions {
          container: container_name.to_owned(),
          ..Default::default()
        };
        docker_api
          .connect_network(&network_name, config)
          .await
          .map_err(|err| HttpError {
            msg: format!("unable to connect container to network {:?}", err),
            status: StatusCode::INTERNAL_SERVER_ERROR,
          })?;

        Ok::<(), HttpError>(())
      })
      .collect::<FuturesUnordered<_>>()
      .collect::<Vec<_>>()
      .await;

    vec_futures
      .into_iter()
      .collect::<Result<Vec<()>, HttpError>>()?;
  }
  Ok(())
}
