use futures::{stream, StreamExt};
use ntex::web;
use serde::{Serialize, Deserialize};

use crate::config::DaemonConfig;
use crate::{services, repositories};
use crate::models::Pool;
use crate::errors::HttpResponseError;
use crate::services::cluster::JoinCargoOptions;

use super::utils::gen_nsp_key_by_name;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterCargoQuery {
  namespace: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterCargoPatchPath {
  cluster_name: String,
  cargo_name: String,
}

#[web::patch("/clusters/{cluster_name}/cargoes/{cargo_name}")]
async fn update_cluster_cargo_by_name(
  req_path: web::types::Path<ClusterCargoPatchPath>,
  daemon_config: web::types::State<DaemonConfig>,
  pool: web::types::State<Pool>,
  docker_api: web::types::State<bollard::Docker>,
  web::types::Query(qs): web::types::Query<ClusterCargoQuery>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let cluster_key = gen_nsp_key_by_name(&qs.namespace, &req_path.cluster_name);
  let cargo_key = gen_nsp_key_by_name(&qs.namespace, &req_path.cargo_name);

  let cluster_cargo = repositories::cluster_cargo::get_by_key(
    format!("{}-{}", &cluster_key, &cargo_key),
    &pool,
  )
  .await?;

  println!("cluster_cargo : {:#?}", &cluster_cargo);

  let network = repositories::cluster_network::find_by_key(
    cluster_cargo.network_key,
    &pool,
  )
  .await?;

  let cluster =
    repositories::cluster::find_by_key(cluster_key.to_owned(), &pool).await?;
  let cargo =
    repositories::cargo::find_by_key(cargo_key.to_owned(), &pool).await?;
  let cnt_to_remove =
    services::cluster::list_containers(&cluster_key, &cargo_key, &docker_api)
      .await?;

  println!("container to remove {:#?}", &cnt_to_remove);

  let opts = JoinCargoOptions {
    cluster: cluster.to_owned(),
    cargo,
    network,
    is_creating_relation: false,
  };

  services::cluster::join_cargo(&opts, &docker_api, &pool).await?;

  services::cluster::start(&cluster, &daemon_config, &pool, &docker_api)
    .await?;

  let mut stream = stream::iter(cnt_to_remove);

  while let Some(container) = stream.next().await {
    let options = Some(bollard::container::RemoveContainerOptions {
      force: true,
      ..Default::default()
    });
    println!("removing container {:#?}", &container);
    docker_api
      .remove_container(&container.id.clone().unwrap_or_default(), options)
      .await?;
  }

  Ok(web::HttpResponse::Ok().into())
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(update_cluster_cargo_by_name);
}
