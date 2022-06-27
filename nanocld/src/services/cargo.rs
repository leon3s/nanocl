use std::io::prelude::*;
use ntex::web;
use ntex::http::StatusCode;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use futures::StreamExt;
use futures::stream::FuturesUnordered;
use crate::models::{
  Pool, CargoItem, CargoPortItem, ClusterItem, CargoProxyConfigItem,
};

use crate::repositories::{cargo_port, cargo_proxy_config, nginx_template};
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

  let mut labels: HashMap<String, String> = HashMap::new();
  labels.insert(String::from("namespace"), item.namespace_name.to_owned());
  labels.insert(String::from("cargo"), item.key.to_owned());
  let config = bollard::container::Config {
    image,
    tty: Some(true),
    labels: Some(labels),
    attach_stdout: Some(true),
    attach_stderr: Some(true),
    host_config: Some(bollard::models::HostConfig {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MustacheData {
  domain_name: String,
  host_ip: String,
  target_ip: String,
}

async fn deploy_proxy_config(
  container_name: &String,
  network_name: &String,
  item: CargoItem,
  proxy_config: CargoProxyConfigItem,
  docker_api: &web::types::State<bollard::Docker>,
  pool: &web::types::State<Pool>,
) -> Result<(), HttpError> {
  let template =
    nginx_template::get_by_name(String::from("nodejs-single"), pool).await?;
  let template =
    mustache::compile_str(&template.content).map_err(|err| HttpError {
      msg: format!("mustache template error: {:?}", err),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

  let container = docker_api
    .inspect_container(container_name, None)
    .await
    .map_err(|err| HttpError {
      msg: format!("unable to inspect container {} {:?}", container_name, err),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

  let networks = container.network_settings.unwrap().networks.unwrap();
  println!("{:?}", networks);
  let container_ip = networks
    .get(network_name)
    .unwrap()
    .ip_address
    .as_ref()
    .unwrap();

  let data = MustacheData {
    domain_name: proxy_config.domain_name,
    host_ip: proxy_config.host_ip,
    target_ip: container_ip.clone(),
  };
  let mut file = std::fs::File::create(format!(
    "/var/lib/nanocl/nginx/sites-enabled/{name}.conf",
    name = item.name
  ))
  .map_err(|err| HttpError {
    msg: format!("unable to generate template file {:?}", err),
    status: StatusCode::INTERNAL_SERVER_ERROR,
  })?;
  template.render(&mut file, &data).unwrap();
  Ok(())
}

pub async fn start_cargo_in_cluster(
  item: CargoItem,
  cluster: ClusterItem,
  docker_api: &web::types::State<bollard::Docker>,
  pool: &web::types::State<Pool>,
) -> Result<(), HttpError> {
  let cargo = &item.to_owned();
  let cargo_key = &cargo.key;
  let cluster_key = &cluster.key;
  let network_name = item.network_name.to_owned();
  let container_names = start_cargo(item, docker_api, pool).await?;

  if let Some(network_name) = &network_name {
    let vec_futures = container_names
      .into_iter()
      .map(|container_name| async move {
        let network_name = cluster_key.to_owned() + "-" + network_name;
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
        let proxy_config =
          cargo_proxy_config::get_for_cargo(cargo_key.to_owned(), pool).await?;
        deploy_proxy_config(
          &container_name,
          &network_name,
          cargo.to_owned(),
          proxy_config,
          docker_api,
          pool,
        )
        .await?;
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
