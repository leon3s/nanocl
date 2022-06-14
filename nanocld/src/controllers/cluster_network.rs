use ntex::web;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use bollard::network::ListNetworksOptions;

use crate::repositories::cluster;
use crate::models::{Pool, Docker};

use super::errors::HttpError;

#[derive(Debug, Serialize, Deserialize)]
struct ClusterQuery {
  pub(crate) namespace: Option<String>,
}

#[utoipa::path(
  get,
  path = "/clusters/{id_or_name}/networks",
  params(
    ("id_or_name" = String, path, description = "Id or name of the cluster"),
    ("namespace" = Option<String>, query, description = "Namespace to add cluster in if empty we use 'default' as value"),
  ),
  responses(
    (status = 201, description = "Fresh created cluster", body = Network),
    (status = 400, description = "Generic database error"),
    (status = 404, description = "Namespace name not valid"),
  ),
)]
#[web::get("/clusters/{id_or_name}/networks")]
async fn list_networks(
  pool: web::types::State<Pool>,
  docker: web::types::State<Docker>,
  id_or_name: web::types::Path<String>,
  web::types::Query(qs): web::types::Query<ClusterQuery>,
) -> Result<web::HttpResponse, HttpError> {
  let id = id_or_name.into_inner();
  let namespace = match qs.namespace {
    None => String::from("default"),
    Some(namespace) => namespace,
  };

  let _cluster = match Uuid::parse_str(&id) {
    Err(_) => {
      cluster::find_by_gen_id(namespace.to_owned() + &id, &pool).await?
    }
    Ok(uuid) => cluster::find_by_id(uuid, &pool).await?,
  };

  // let mut list_networks_filters = HashMap::new();
  // list_networks_filters.insert("label", vec!["maintainer=some_maintainer"]);
  let config = ListNetworksOptions::<&str> {
    // filters: list_networks_filters,
    ..Default::default()
  };
  let res = docker.list_networks(Some(config)).await.unwrap();
  Ok(web::HttpResponse::Ok().json(&res))
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(list_networks);
}

#[cfg(test)]
mod test_cluster_network {
  use crate::utils::test::*;

  use super::*;

  async fn list_networks(srv: &TestServer) -> TestReturn {
    let mut res = srv.get("/clusters/default/networks").send().await?;

    let _body = res.body().await;
    Ok(())
  }

  #[ntex::test]
  async fn main() -> TestReturn {
    let srv = generate_server(ntex_config);

    list_networks(&srv).await?;
    Ok(())
  }
}
