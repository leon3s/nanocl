use ntex::web;
use ntex::http::StatusCode;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use futures::{StreamExt, TryStreamExt};
use futures::stream::FuturesUnordered;

use crate::{services, repositories};
use crate::models::{
  Pool, ClusterItem, CargoItem, ClusterNetworkItem, ClusterCargoPartial,
};

use crate::controllers::errors::{HttpError, IntoHttpError};

use super::errors::docker_error;

#[derive(Debug)]
pub struct JoinCargoOptions {
  pub(crate) cluster: ClusterItem,
  pub(crate) cargo: CargoItem,
  pub(crate) network: ClusterNetworkItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NginxTemplateData {
  domain_name: String,
  host_ip: String,
  target_ip: String,
  target_ips: Vec<String>,
  target_port: i32,
}

pub async fn start(
  cluster: &ClusterItem,
  docker_api: &web::types::State<bollard::Docker>,
  pool: &web::types::State<Pool>,
) -> Result<(), HttpError> {
  let cluster_cargoes = repositories::cluster_cargo::get_by_cluster_key(
    cluster.key.to_owned(),
    pool,
  )
  .await?;

  cluster_cargoes
    .into_iter()
    .map(|cluster_cargo| async move {
      let cargo_key = &cluster_cargo.cargo_key;
      let network_key = &cluster_cargo.network_key;
      let containers =
        services::cargo::list_containers(cargo_key.to_owned(), docker_api)
          .await?;

      println!("starting cargo {}", &cargo_key);

      let target_ips = containers
        .into_iter()
        .map(|container| async move {
          let container_id = container.id.unwrap_or_default();
          println!("starting container {}", &container_id);
          docker_api
            .start_container(
              &container_id,
              None::<bollard::container::StartContainerOptions<String>>,
            )
            .await
            .map_err(docker_error)?;
          println!("started");
          let container = docker_api
            .inspect_container(&container_id, None)
            .await
            .map_err(docker_error)?;
          let networks = container
            .network_settings
            .ok_or(HttpError {
              msg: format!(
                "unable to get network settings for container {:#?}",
                &container_id,
              ),
              status: StatusCode::INTERNAL_SERVER_ERROR,
            })?
            .networks
            .ok_or(HttpError {
              msg: format!(
                "unable to get networks for container {:#?}",
                &container_id
              ),
              status: StatusCode::INTERNAL_SERVER_ERROR,
            })?;
          let network = networks.get(network_key).ok_or(HttpError {
            msg: format!(
              "unable to get network {} for container {}",
              &network_key, &container_id
            ),
            status: StatusCode::INTERNAL_SERVER_ERROR,
          })?;
          let ip_address = network.ip_address.as_ref().ok_or(HttpError {
            msg: format!(
              "unable to get ip_address of container {}",
              &container_id
            ),
            status: StatusCode::INTERNAL_SERVER_ERROR,
          })?;
          Ok::<String, HttpError>(ip_address.into())
        })
        .collect::<FuturesUnordered<_>>()
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<String>, HttpError>>()?;
      println!("setup proxy config");
      let proxy_config =
        repositories::cargo_proxy_config::get_for_cargo(cargo_key.into(), pool)
          .await;
      if let Ok(proxy_config) = proxy_config {
        let template = repositories::nginx_template::get_by_name(
          String::from("nodejs-single"),
          pool,
        )
        .await?;
        let content = &template.content;
        let template =
          mustache::compile_str(content).map_err(|err| HttpError {
            msg: format!("mustache template error: {:?}", err),
            status: StatusCode::INTERNAL_SERVER_ERROR,
          })?;
        let data = NginxTemplateData {
          domain_name: proxy_config.domain_name.to_owned(),
          host_ip: proxy_config.host_ip.to_owned(),
          target_ip: target_ips[0].to_owned(),
          target_ips,
          target_port: proxy_config.target_port,
        };
        log::debug!("generating nginx template with content : {:#?}", content);
        log::debug!("generating nginx template with data : {:#?}", &data);
        let mut file = std::fs::File::create(format!(
          "/var/lib/nanocl/nginx/sites-enabled/{name}.conf",
          name = &cargo_key
        ))
        .map_err(|err| HttpError {
          msg: format!("unable to generate template file {:?}", err),
          status: StatusCode::INTERNAL_SERVER_ERROR,
        })?;
        template.render(&mut file, &data).map_err(|err| HttpError {
          msg: format!(
            "unable to render nginx template for cargo {} : {:#?}",
            &cargo_key, err
          ),
          status: StatusCode::INTERNAL_SERVER_ERROR,
        })?;
        services::nginx::reload_config(docker_api)
          .await
          .map_err(docker_error)?;
        services::dnsmasq::add_dns_entry(
          &proxy_config.domain_name,
          &proxy_config.host_ip,
        )
        .map_err(|err| err.to_http_error())?;
        services::dnsmasq::restart(docker_api)
          .await
          .map_err(|err| err.to_http_error())?;
      }
      Ok::<_, HttpError>(())
    })
    .collect::<FuturesUnordered<_>>()
    .collect::<Vec<_>>()
    .await
    .into_iter()
    .collect::<Result<Vec<()>, HttpError>>()?;
  Ok(())
}

pub async fn join_cargo(
  opts: &JoinCargoOptions,
  docker_api: &web::types::State<bollard::Docker>,
  pool: &web::types::State<Pool>,
) -> Result<(), HttpError> {
  let cluster_cargo = ClusterCargoPartial {
    cluster_key: opts.cluster.key.to_owned(),
    cargo_key: opts.cargo.key.to_owned(),
    network_key: opts.network.key.to_owned(),
  };
  repositories::cluster_cargo::create(cluster_cargo, pool).await?;

  let mut labels: HashMap<String, String> = HashMap::new();
  labels.insert(String::from("cluster"), opts.cluster.key.to_owned());
  let container_ids = services::cargo::create_containers(
    &opts.cargo,
    opts.network.key.to_owned(),
    Some(&mut labels),
    docker_api,
  )
  .await?;

  container_ids
    .into_iter()
    .map(|container_name| async move {
      let config = bollard::network::ConnectNetworkOptions {
        container: container_name.to_owned(),
        ..Default::default()
      };
      docker_api
        .connect_network(&opts.network.key, config)
        .await
        .map_err(docker_error)?;
      Ok::<(), HttpError>(())
    })
    .collect::<FuturesUnordered<_>>()
    .collect::<Vec<_>>()
    .await
    .into_iter()
    .collect::<Result<Vec<()>, HttpError>>()?;

  Ok(())
}
