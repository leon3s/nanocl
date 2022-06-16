//! File to handle cluster routes
use ntex::web;
use serde::{Deserialize, Serialize};

use crate::models::{ClusterPartial, Pool};
use crate::repositories::cluster;

use super::errors::HttpError;

#[derive(Debug, Serialize, Deserialize)]
struct ClusterQuery {
  pub(crate) namespace: Option<String>,
}

/// List all cluster
#[utoipa::path(
  get,
  path = "/clusters",
  params(
    ("namespace" = Option<String>, query, description = "Namespace to add cluster in if empty we use 'default' as value"),
  ),
  responses(
    (status = 200, description = "List of cluster for given namespace", body = ClusterItem),
    (status = 400, description = "Generic database error"),
    (status = 404, description = "Namespace name not valid"),
  ),
)]
#[web::get("/clusters")]
async fn list_cluster(
  pool: web::types::State<Pool>,
  web::types::Query(qs): web::types::Query<ClusterQuery>,
) -> Result<web::HttpResponse, HttpError> {
  let nsp = match qs.namespace {
    None => String::from("default"),
    Some(namespace) => namespace,
  };

  let items = cluster::find_by_namespace(nsp, &pool).await?;
  Ok(web::HttpResponse::Ok().json(&items))
}

/// Create new cluster
#[utoipa::path(
  post,
  request_body = ClusterPartial,
  path = "/clusters",
  params(
    ("namespace" = Option<String>, query, description = "Namespace to add cluster in if empty we use 'default' as value"),
  ),
  responses(
    (status = 201, description = "Fresh created cluster", body = ClusterItem),
    (status = 400, description = "Generic database error"),
    (status = 404, description = "Namespace name not valid"),
  ),
)]
#[web::post("/clusters")]
async fn create_cluster(
  pool: web::types::State<Pool>,
  web::types::Query(qs): web::types::Query<ClusterQuery>,
  web::types::Json(json): web::types::Json<ClusterPartial>,
) -> Result<web::HttpResponse, HttpError> {
  let nsp = match qs.namespace {
    None => String::from("default"),
    Some(namespace) => namespace,
  };
  let res = cluster::create_for_namespace(nsp, json, &pool).await?;
  Ok(web::HttpResponse::Created().json(&res))
}

/// Delete cluster by it's name
#[utoipa::path(
  delete,
  path = "/clusters/{name}",
  params(
    ("name" = String, path, description = "Name of the cluster"),
    ("namespace" = Option<String>, query, description = "Namespace to add cluster in if empty we use 'default' as value"),
  ),
  responses(
    (status = 201, description = "Fresh created cluster", body = ClusterItem),
    (status = 400, description = "Generic database error", body = ApiError),
    (status = 404, description = "Namespace name not valid", body = ApiError),
  ),
)]
#[web::delete("clusters/{name}")]
async fn delete_cluster_by_name(
  pool: web::types::State<Pool>,
  name: web::types::Path<String>,
  web::types::Query(qs): web::types::Query<ClusterQuery>,
) -> Result<web::HttpResponse, HttpError> {
  let nsp = match qs.namespace {
    None => String::from("default"),
    Some(namespace) => namespace,
  };
  let gen_key = nsp.to_owned() + "-" + &name.into_inner();
  let res = cluster::delete_by_key(gen_key, &pool).await?;
  Ok(web::HttpResponse::Ok().json(&res))
}

/// Inspect cluster by it's name
#[utoipa::path(
  get,
  path = "/clusters/{name}/inspect",
  params(
    ("name" = String, path, description = "Name of the cluster"),
    ("namespace" = Option<String>, query, description = "Namespace to add cluster in if empty we use 'default' as value"),
  ),
  responses(
    (status = 200, description = "Cluster information", body = ClusterItem),
    (status = 400, description = "Generic database error", body = ApiError),
    (status = 404, description = "id name or namespace name not valid", body = ApiError),
  ),
)]
#[web::get("/clusters/{name}/inspect")]
async fn inspect_cluster_by_name(
  pool: web::types::State<Pool>,
  name: web::types::Path<String>,
  web::types::Query(qs): web::types::Query<ClusterQuery>,
) -> Result<web::HttpResponse, HttpError> {
  let nsp = match qs.namespace {
    None => String::from("default"),
    Some(namespace) => namespace,
  };
  let gen_key = nsp.to_owned() + "-" + &name.into_inner();
  let item = cluster::find_by_key(gen_key, &pool).await?;
  Ok(web::HttpResponse::Ok().json(&item))
}

/// # ntex config
/// Bind namespace routes to ntex http server
///
/// # Arguments
/// [config](web::ServiceConfig) mutable service config
///
/// # Examples
/// ```rust,norun
/// use ntex::web;
/// use crate::controllers;
///
/// web::App::new().configure(controllers::cluster::ntex_config)
/// ```
pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(list_cluster);
  config.service(create_cluster);
  config.service(inspect_cluster_by_name);
  config.service(delete_cluster_by_name);
}

#[cfg(test)]
mod test_namespace_cluster {
  use crate::utils::test::*;

  use super::*;

  async fn test_list(srv: &TestServer) -> TestReturn {
    let resp = srv.get("/clusters").send().await?;

    assert!(resp.status().is_success());
    Ok(())
  }

  async fn test_list_with_nsp(srv: &TestServer) -> TestReturn {
    let resp = srv
      .get("/clusters")
      .query(&ClusterQuery {
        namespace: Some(String::from("test")),
      })?
      .send()
      .await?;

    assert!(resp.status().is_success());
    Ok(())
  }

  async fn test_create(srv: &TestServer) -> TestReturn {
    let item = ClusterPartial {
      name: String::from("test_cluster"),
    };
    let resp = srv.post("/clusters").send_json(&item).await?;

    assert!(resp.status().is_success());
    Ok(())
  }

  async fn test_delete(srv: &TestServer) -> TestReturn {
    let resp = srv.delete("/clusters/test_cluster").send().await?;
    assert!(resp.status().is_success());
    Ok(())
  }

  #[ntex::test]
  async fn main() -> TestReturn {
    let srv = generate_server(ntex_config);
    test_list(&srv).await?;
    test_list_with_nsp(&srv).await?;
    test_create(&srv).await?;
    test_delete(&srv).await?;
    Ok(())
  }
}
