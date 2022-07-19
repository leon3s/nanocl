use ntex::web;
use ntex::http::StatusCode;
use std::collections::HashMap;
use std::path::Path;
use serde::{Serialize, Deserialize};
use futures::{StreamExt, stream};
use futures::stream::FuturesUnordered;

use crate::config::DaemonConfig;
use crate::utils::render_template;
use crate::{services, repositories};
use crate::models::{
  Pool, ClusterItem, CargoItem, ClusterNetworkItem, ClusterCargoPartial,
  CargoEnvItem, NginxTemplateModes, ClusterCargoItem,
};

use crate::errors::{HttpResponseError, IntoHttpResponseError};

use super::cargo::CreateCargoContainerOpts;

#[derive(Debug)]
pub struct JoinCargoOptions {
  pub(crate) cluster: ClusterItem,
  pub(crate) cargo: CargoItem,
  pub(crate) network: ClusterNetworkItem,
  pub(crate) is_creating_relation: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkTemplateData {
  pub(crate) gateway: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateData {
  vars: Option<HashMap<String, String>>,
  cargoes: HashMap<String, CargoTemplateData>,
  networks: Option<HashMap<String, NetworkTemplateData>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoTemplateData {
  name: String,
  target_ip: String,
  dns_entry: Option<String>,
  target_ips: Vec<String>,
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
      let _ = docker_api
        .remove_network(&network.docker_network_id)
        .await
        .map_err(|err| HttpResponseError {
          msg: format!("unable to remove network {:#?}", err),
          status: StatusCode::INTERNAL_SERVER_ERROR,
        });
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

async fn start_containers(
  containers: Vec<bollard::models::ContainerSummary>,
  network_key: &str,
  docker_api: &web::types::State<bollard::Docker>,
) -> Result<Vec<String>, HttpResponseError> {
  log::info!("Starting cargoes");
  let target_ips = containers
    .into_iter()
    .map(|container| async move {
      let container_id = container.id.unwrap_or_default();
      log::info!("starting container {}", &container_id);
      let state = container.state.unwrap_or_default();
      if state != "running" {
        docker_api
          .start_container(
            &container_id,
            None::<bollard::container::StartContainerOptions<String>>,
          )
          .await?;
      }
      log::info!("successfully started container {}", &container_id);
      let container = docker_api.inspect_container(&container_id, None).await?;
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
  log::info!("all cargo started");
  Ok(target_ips)
}

async fn start_cluster_cargoes(
  cluster_cargoes: Vec<ClusterCargoItem>,
  docker_api: &web::types::State<bollard::Docker>,
  pool: &web::types::State<Pool>,
) -> Result<Vec<CargoTemplateData>, HttpResponseError> {
  cluster_cargoes
    .into_iter()
    .map(|cluster_cargo| async move {
      let cargo_key = &cluster_cargo.cargo_key;
      let network_key = &cluster_cargo.network_key;
      let containers = list_containers(
        &cluster_cargo.cluster_key,
        &cluster_cargo.cargo_key,
        docker_api,
      )
      .await?;

      let cargo =
        repositories::cargo::find_by_key(cargo_key.to_owned(), pool).await?;

      let mut target_ips =
        start_containers(containers, network_key, docker_api).await?;
      target_ips.reverse();
      let target_ip = match target_ips.get(0) {
        None => String::new(),
        Some(target_ip) => target_ip.to_owned(),
      };
      let cargo_template_data = CargoTemplateData {
        name: cargo.name,
        dns_entry: cargo.dns_entry,
        target_ip,
        target_ips,
      };
      Ok::<CargoTemplateData, HttpResponseError>(cargo_template_data)
    })
    .collect::<FuturesUnordered<_>>()
    .collect::<Vec<_>>()
    .await
    .into_iter()
    .collect::<Result<Vec<CargoTemplateData>, HttpResponseError>>()
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

  let cargoes = start_cluster_cargoes(cluster_cargoes, docker_api, pool)
    .await?
    .into_iter()
    .fold(HashMap::new(), |mut acc, item| {
      acc.insert(item.name.to_owned(), item);
      acc
    });

  if !cluster.proxy_templates.is_empty() {
    let cluster_vars = repositories::cluster_variable::list_by_cluster(
      cluster.key.to_owned(),
      pool,
    )
    .await?;
    let vars =
      services::cluster_variable::cluster_vars_to_hashmap(cluster_vars);

    let networks =
      repositories::cluster_network::list_for_cluster(cluster.to_owned(), pool)
        .await?
        .into_iter()
        .fold(HashMap::new(), |mut acc, network| {
          acc.insert(
            network.name.to_owned(),
            NetworkTemplateData {
              gateway: network.default_gateway,
            },
          );
          acc
        });

    let mut templates = stream::iter(&cluster.proxy_templates);

    while let Some(template_name) = templates.next().await {
      let template = repositories::nginx_template::get_by_name(
        template_name.to_owned(),
        pool,
      )
      .await?;
      let file_path = Path::new(&config.state_dir);
      let file_path = match template.mode {
        NginxTemplateModes::Http => file_path.join("nginx/sites-enabled"),
        NginxTemplateModes::Stream => file_path.join("nginx/streams-enabled"),
      };
      let file_path =
        file_path.join(format!("{name}.conf", name = &cluster.key));
      let template_data = TemplateData {
        vars: Some(vars.to_owned()),
        networks: Some(networks.to_owned()),
        cargoes: cargoes.to_owned(),
      };

      let config_file = render_template(template.content, &template_data)?;
      std::fs::write(&file_path, config_file).map_err(|err| {
        HttpResponseError {
          msg: format!(
            "Unable to write config file {} {}",
            &file_path.display(),
            err
          ),
          status: StatusCode::INTERNAL_SERVER_ERROR,
        }
      })?;

      let mut cargoes = stream::iter(&cargoes);

      println!("{:#?}", &networks);

      while let Some((_, item)) = cargoes.next().await {
        if None == item.dns_entry {
          continue;
        }
        let item_string =
          serde_json::to_string(&item).map_err(|err| HttpResponseError {
            msg: format!("{}", err),
            status: StatusCode::INTERNAL_SERVER_ERROR,
          })?;

        let item: CargoTemplateData =
          serde_json::from_str(&render_template(item_string, &template_data)?)
            .map_err(|err| HttpResponseError {
              msg: format!("{}", err),
              status: StatusCode::INTERNAL_SERVER_ERROR,
            })?;

        let domain = item.dns_entry.ok_or(HttpResponseError {
          msg: String::from("Unexpected error domain should not be null"),
          status: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

        let dns_settings = domain.split(':').collect::<Vec<_>>();

        if dns_settings.len() != 2 {
          return Err(HttpResponseError {
            msg: String::from("Error dns settings have incorrect format"),
            status: StatusCode::BAD_REQUEST,
          });
        }

        services::dnsmasq::add_dns_entry(
          dns_settings[1],
          dns_settings[0],
          &config.state_dir,
        )
        .map_err(|err| err.to_http_error())?;
      }

      services::dnsmasq::restart(docker_api)
        .await
        .map_err(|err| err.to_http_error())?;
      services::nginx::reload_config(docker_api).await?;
    }
  }
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
) -> Result<Vec<String>, HttpResponseError> {
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
    network_key: &opts.network.key,
    cluster_name: &opts.cluster.name,
    labels: Some(&mut labels),
    environnements,
  };

  let container_ids =
    services::cargo::create_containers(create_opts, docker_api).await?;

  container_ids
    .clone()
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

  if opts.is_creating_relation {
    repositories::cluster_cargo::create(cluster_cargo, pool).await?;
  }

  Ok(container_ids)
}
