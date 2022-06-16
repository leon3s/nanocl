use std::collections::HashMap;

use ntex::{web, http::StatusCode};
use serde::{Serialize, Deserialize};

use super::errors::HttpError;
use crate::repositories::{cluster, cluster_network};
use crate::models::{ClusterNetworkPartial, Pool};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNetworkQuery {
  pub(crate) namespace: Option<String>,
}

/// List network for given cluster
#[utoipa::path(
  get,
  path = "/clusters/{name}/networks",
  params(
    ("name" = String, path, description = "name of the cluster"),
    ("namespace" = Option<String>, query, description = "Namespace to add cluster in if empty we use 'default' as value"),
  ),
  responses(
    (status = 201, description = "List of networks", body = [ClusterNetworkItem]),
    (status = 400, description = "Generic database error", body = ApiError),
    (status = 404, description = "Namespace name not valid", body = ApiError),
  ),
)]
#[web::get("/clusters/{name}/networks")]
async fn list_cluster_network(
  name: web::types::Path<String>,
  web::types::Query(qs): web::types::Query<ClusterNetworkQuery>,
) -> Result<web::HttpResponse, HttpError> {
  let nsp = match qs.namespace {
    None => String::from("default"),
    Some(nsp) => nsp,
  };

  // qs.cluster
  Ok(web::HttpResponse::Ok().into())
}

/// Create a network for given cluster
#[utoipa::path(
  post,
  request_body = ClusterNetworkPartial,
  path = "/clusters/{name}/networks",
  params(
    ("name" = String, path, description = "name of the cluster"),
    ("namespace" = Option<String>, query, description = "Namespace to add cluster in if empty we use 'default' as value"),
  ),
  responses(
    (status = 201, description = "List of networks", body = [ClusterNetworkItem]),
    (status = 400, description = "Generic database error", body = ApiError),
    (status = 404, description = "Namespace name not valid", body = ApiError),
  ),
)]
#[web::post("/clusters/{name}/networks")]
async fn create_cluster_network(
  pool: web::types::State<Pool>,
  docker: web::types::State<bollard::Docker>,
  name: web::types::Path<String>,
  web::types::Query(qs): web::types::Query<ClusterNetworkQuery>,
  web::types::Json(payload): web::types::Json<ClusterNetworkPartial>,
) -> Result<web::HttpResponse, HttpError> {
  let name = name.into_inner();
  let nsp = match qs.namespace {
    None => String::from("default"),
    Some(nsp) => nsp,
  };
  let gen_key = nsp + "-" + &name;
  let cluster = cluster::find_by_key(gen_key.clone(), &pool).await?;
  let mut labels = HashMap::new();
  labels.insert(String::from("cluster_key"), gen_key.clone());
  let gen_name = cluster.key.to_owned() + "-" + &payload.name;
  let network_existing =
    match cluster_network::find_by_key(gen_name.clone(), &pool).await {
      Err(_) => false,
      Ok(_) => true,
    };
  if network_existing {
    return Err(HttpError {
      status: StatusCode::BAD_REQUEST,
      msg: format!("Unable to create network with name {} a similar network have same name", name),
    });
  }
  let config = bollard::network::CreateNetworkOptions {
    name: gen_name,
    labels,
    ..Default::default()
  };
  let id = match docker.create_network(config).await {
    Err(_) => {
      return Err(HttpError {
        status: StatusCode::BAD_REQUEST,
        msg: format!("Unable to create network with name {}", name),
      })
    }
    Ok(result) => result.id,
  };
  let id = match id {
    None => {
      return Err(HttpError {
        status: StatusCode::BAD_REQUEST,
        msg: format!("Unable to create network with name {}", name),
      })
    }
    Some(id) => id,
  };

  let new_network =
    cluster_network::create_for_cluster(gen_key, payload, id, &pool).await?;
  // qs.cluster
  Ok(web::HttpResponse::Created().json(&new_network))
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(list_cluster_network);
  config.service(create_cluster_network);
}
