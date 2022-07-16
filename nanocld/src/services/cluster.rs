use ntex::web;
use ntex::http::StatusCode;
use std::collections::HashMap;
use std::path::Path;
use serde::{Serialize, Deserialize};
use futures::StreamExt;
use futures::stream::FuturesUnordered;

use crate::config::DaemonConfig;
use crate::{services, repositories};
use crate::models::{
  Pool, ClusterItem, CargoItem, ClusterNetworkItem, ClusterCargoPartial,
  CargoEnvItem, NginxTemplateModes, CargoProxyConfigItem,
};

use crate::errors::{HttpResponseError, IntoHttpResponseError};

use super::cargo::CreateCargoContainerOpts;

#[derive(Debug)]
pub struct JoinCargoOptions {
  pub(crate) cluster: ClusterItem,
  pub(crate) cargo: CargoItem,
  pub(crate) network: ClusterNetworkItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkTemplateData {
  pub(crate) gateway: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NginxTemplateData {
  domain_name: String,
  host_ip: String,
  target_ip: String,
  target_ips: Vec<String>,
  target_port: i32,
  network: NetworkTemplateData,
  vars: Option<HashMap<String, String>>,
}

pub async fn delete_networks(
  cluster: ClusterItem,
  docker_api: &web::types::State<bollard::Docker>,
  pool: &web::types::State<Pool>,
) -> Result<(), HttpResponseError> {
  let networks =
    repositories::cluster_network::list_for_cluster(cluster, pool).await?;

  networks
    .into_iter()
    .map(|network| async move {
      docker_api
        .remove_network(&network.docker_network_id)
        .await
        .map_err(|err| HttpResponseError {
          msg: format!("unable to remove network {:#?}", err),
          status: StatusCode::INTERNAL_SERVER_ERROR,
        })?;
      repositories::cluster_network::delete_by_key(network.key, pool).await?;
      Ok::<_, HttpResponseError>(())
    })
    .collect::<FuturesUnordered<_>>()
    .collect::<Vec<_>>()
    .await
    .into_iter()
    .collect::<Result<Vec<_>, HttpResponseError>>()?;

  Ok(())
}

pub async fn list_containers(
  cluster_key: &str,
  cargo_key: &str,
  docker_api: &web::types::State<bollard::Docker>,
) -> Result<Vec<bollard::models::ContainerSummary>, HttpResponseError> {
  let target_cluster = &format!("cluster={}", &cluster_key);
  let target_cargo = &format!("cargo={}", &cargo_key);
  let mut filters = HashMap::new();
  filters.insert(
    "label",
    vec![target_cluster.as_str(), target_cargo.as_str()],
  );
  let options = Some(bollard::container::ListContainersOptions {
    all: true,
    filters,
    ..Default::default()
  });
  let containers = docker_api.list_containers(options).await?;
  Ok(containers)
}

pub async fn start(
  cluster: &ClusterItem,
  config: &DaemonConfig,
  pool: &web::types::State<Pool>,
  docker_api: &web::types::State<bollard::Docker>,
) -> Result<(), HttpResponseError> {
  let cluster_cargoes = repositories::cluster_cargo::get_by_cluster_key(
    cluster.key.to_owned(),
    pool,
  )
  .await?;

  let cluster_vars = repositories::cluster_variable::list_by_cluster(
    cluster.key.to_owned(),
    pool,
  )
  .await?;

  let vars = &services::cluster_variable::cluster_vars_to_hashmap(cluster_vars);

  cluster_cargoes
    .into_iter()
    .map(|cluster_cargo| async move {
      let cluster_cargo_key = &cluster_cargo.key;
      let cargo_key = &cluster_cargo.cargo_key;
      let network_key = &cluster_cargo.network_key;
      let containers = list_containers(
        &cluster_cargo.cluster_key,
        &cluster_cargo.cargo_key,
        docker_api,
      )
      .await?;

      log::info!("starting cargo {}", &cargo_key);

      let target_ips = containers
        .into_iter()
        .map(|container| async move {
          let container_id = container.id.unwrap_or_default();
          log::info!("starting container {}", &container_id);
          docker_api
            .start_container(
              &container_id,
              None::<bollard::container::StartContainerOptions<String>>,
            )
            .await?;
          log::info!("successfully started container {}", &container_id);
          let container =
            docker_api.inspect_container(&container_id, None).await?;
          let networks = container
            .network_settings
            .ok_or(HttpResponseError {
              msg: format!(
                "unable to get network settings for container {:#?}",
                &container_id,
              ),
              status: StatusCode::INTERNAL_SERVER_ERROR,
            })?
            .networks
            .ok_or(HttpResponseError {
              msg: format!(
                "unable to get networks for container {:#?}",
                &container_id
              ),
              status: StatusCode::INTERNAL_SERVER_ERROR,
            })?;
          let network = networks.get(network_key).ok_or(HttpResponseError {
            msg: format!(
              "unable to get network {} for container {}",
              &network_key, &container_id
            ),
            status: StatusCode::INTERNAL_SERVER_ERROR,
          })?;
          let ip_address =
            network.ip_address.as_ref().ok_or(HttpResponseError {
              msg: format!(
                "unable to get ip_address of container {}",
                &container_id
              ),
              status: StatusCode::INTERNAL_SERVER_ERROR,
            })?;
          Ok::<String, HttpResponseError>(ip_address.into())
        })
        .collect::<FuturesUnordered<_>>()
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<String>, HttpResponseError>>()?;
      let proxy_config =
        repositories::cargo_proxy_config::get_for_cargo(cargo_key.into(), pool)
          .await;
      log::info!("setup proxy config {:#?}", &proxy_config);
      if let Ok(proxy_config) = proxy_config {
        let network = repositories::cluster_network::find_by_key(
          network_key.to_owned(),
          pool,
        )
        .await?;

        let target_ip = target_ips[0].to_owned();

        let data = NginxTemplateData {
          domain_name: proxy_config.domain_name.to_owned(),
          host_ip: proxy_config.host_ip.to_owned(),
          target_ip,
          target_ips: target_ips.to_owned(),
          network: NetworkTemplateData {
            gateway: network.default_gateway.to_owned(),
          },
          target_port: proxy_config.target_port,
          vars: Some(vars.to_owned()),
        };

        let proxy_config_content = serde_json::to_string(&proxy_config)
          .map_err(|err| HttpResponseError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            msg: format!("Unable to stringify proxy config {:#?}", err),
          })?;

        let proxy_config_template =
          mustache::compile_str(&proxy_config_content).map_err(|err| {
            HttpResponseError {
              msg: format!("mustache template error: {:?}", err),
              status: StatusCode::INTERNAL_SERVER_ERROR,
            }
          })?;

        let proxy_config_content = proxy_config_template
          .render_to_string(&data)
          .map_err(|err| HttpResponseError {
            msg: format!(
              "Unable to render proxy config template for cargo {} : {:#?}",
              &cargo_key, err
            ),
            status: StatusCode::INTERNAL_SERVER_ERROR,
          })?;

        let proxy_config =
          serde_json::from_str::<CargoProxyConfigItem>(&proxy_config_content)
            .map_err(|err| HttpResponseError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            msg: format!("Unable to serialize proxy config {:#?}", err),
          })?;

        let template = repositories::nginx_template::get_by_name(
          proxy_config.template,
          pool,
        )
        .await?;
        let mustemplate =
          mustache::compile_str(&template.content).map_err(|err| {
            HttpResponseError {
              msg: format!("mustache template error: {:?}", err),
              status: StatusCode::INTERNAL_SERVER_ERROR,
            }
          })?;
        log::debug!(
          "generating nginx template with content : {:#?}",
          &template.content
        );
        log::debug!("generating nginx template with data : {:#?}", &data);

        let file_path = Path::new(&config.state_dir);

        let file_path = match template.mode {
          NginxTemplateModes::Http => file_path.join("nginx/sites-enabled"),
          NginxTemplateModes::Stream => file_path.join("nginx/streams-enabled"),
        };
        let file_path =
          file_path.join(format!("{name}.conf", name = &cluster_cargo_key));

        let mut file = std::fs::File::create(&file_path).map_err(|err| {
          HttpResponseError {
            msg: format!(
              "Unable to generate template file {} {:?}",
              file_path.display(),
              err
            ),
            status: StatusCode::INTERNAL_SERVER_ERROR,
          }
        })?;
        let data = NginxTemplateData {
          domain_name: proxy_config.domain_name.to_owned(),
          host_ip: proxy_config.host_ip.to_owned(),
          target_ip: target_ips[0].to_owned(),
          target_ips,
          network: NetworkTemplateData {
            gateway: network.default_gateway,
          },
          target_port: proxy_config.target_port,
          vars: Some(vars.to_owned()),
        };
        mustemplate.render(&mut file, &data).map_err(|err| {
          HttpResponseError {
            msg: format!(
              "unable to render nginx template for cargo {} : {:#?}",
              &cargo_key, err
            ),
            status: StatusCode::INTERNAL_SERVER_ERROR,
          }
        })?;
        services::nginx::reload_config(docker_api).await?;
        let mut dns_entry = String::new();
        if let Some(pre_domain) = vars.get("pre_domain") {
          dns_entry += &(pre_domain.to_owned() + &proxy_config.domain_name);
        } else {
          dns_entry += &proxy_config.domain_name;
        }
        services::dnsmasq::add_dns_entry(
          &dns_entry,
          &proxy_config.host_ip,
          &config.state_dir,
        )
        .map_err(|err| err.to_http_error())?;
        services::dnsmasq::restart(docker_api)
          .await
          .map_err(|err| err.to_http_error())?;
      }
      Ok::<_, HttpResponseError>(())
    })
    .collect::<FuturesUnordered<_>>()
    .collect::<Vec<_>>()
    .await
    .into_iter()
    .collect::<Result<Vec<()>, HttpResponseError>>()?;
  Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MustacheData {
  pub(crate) vars: HashMap<String, String>,
}

pub async fn join_cargo(
  opts: &JoinCargoOptions,
  docker_api: &web::types::State<bollard::Docker>,
  pool: &web::types::State<Pool>,
) -> Result<(), HttpResponseError> {
  let cluster_cargo = ClusterCargoPartial {
    cluster_key: opts.cluster.key.to_owned(),
    cargo_key: opts.cargo.key.to_owned(),
    network_key: opts.network.key.to_owned(),
  };
  let mut labels: HashMap<String, String> = HashMap::new();
  labels.insert(String::from("cluster"), opts.cluster.key.to_owned());

  let vars = repositories::cluster_variable::list_by_cluster(
    opts.cluster.key.to_owned(),
    pool,
  )
  .await?;
  let envs =
    repositories::cargo_env::list_by_cargo_key(opts.cargo.key.to_owned(), pool)
      .await?;

  let env_string =
    serde_json::to_string(&envs).map_err(|err| HttpResponseError {
      msg: format!("unable to format cargo env items {:#?}", err),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

  let template =
    mustache::compile_str(&env_string).map_err(|err| HttpResponseError {
      msg: format!("unable to compile env_string {:#?}", err),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

  let vars = services::cluster_variable::cluster_vars_to_hashmap(vars);
  let template_data = MustacheData { vars };
  let env_string_with_vars = template
    .render_to_string(&template_data)
    .map_err(|err| HttpResponseError {
      msg: format!("unable to populate env with cluster variables: {:#?}", err),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    })?;
  let envs = serde_json::from_str::<Vec<CargoEnvItem>>(&env_string_with_vars)
    .map_err(|err| HttpResponseError {
    msg: format!("unable to reserialize environements : {:#?}", err),
    status: StatusCode::INTERNAL_SERVER_ERROR,
  })?;
  // template.render_data_to_string(template_data);
  let mut fold_init: Vec<String> = Vec::new();
  let environnements = envs
    .into_iter()
    .fold(&mut fold_init, |acc, item| {
      let s = format!("{}={}", item.name, item.value);
      acc.push(s);
      acc
    })
    .to_vec();
  let create_opts = CreateCargoContainerOpts {
    cargo: &opts.cargo,
    cluster_name: &opts.cluster.name,
    labels: Some(&mut labels),
    environnements,
  };

  let container_ids =
    services::cargo::create_containers(create_opts, docker_api).await?;

  container_ids
    .into_iter()
    .map(|container_name| async move {
      let config = bollard::network::ConnectNetworkOptions {
        container: container_name.to_owned(),
        ..Default::default()
      };
      docker_api
        .connect_network(&opts.network.key, config)
        .await?;
      Ok::<(), HttpResponseError>(())
    })
    .collect::<FuturesUnordered<_>>()
    .collect::<Vec<_>>()
    .await
    .into_iter()
    .collect::<Result<Vec<()>, HttpResponseError>>()?;

  repositories::cluster_cargo::create(cluster_cargo, pool).await?;

  Ok(())
}
