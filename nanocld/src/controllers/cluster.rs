//! File to handle cluster routes
use ntex::web;
use serde::{Deserialize, Serialize};

use crate::config::DaemonConfig;
use crate::services;
use crate::repositories;

use crate::services::cluster::JoinCargoOptions;
use crate::models::{
  Pool, ClusterJoinBody, ClusterPartial, ClusterItemWithRelation,
};

use crate::errors::HttpResponseError;

#[derive(Debug, Serialize, Deserialize)]
struct ClusterQuery {
  pub(crate) namespace: Option<String>,
}

/// List all cluster
#[cfg_attr(feature = "openapi", utoipa::path(
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
))]
#[web::get("/clusters")]
async fn list_cluster(
  pool: web::types::State<Pool>,
  web::types::Query(qs): web::types::Query<ClusterQuery>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let nsp = match qs.namespace {
    None => String::from("global"),
    Some(namespace) => namespace,
  };

  let items = repositories::cluster::find_by_namespace(nsp, &pool).await?;
  Ok(web::HttpResponse::Ok().json(&items))
}

/// Create new cluster
#[cfg_attr(feature = "openapi", utoipa::path(
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
))]
#[web::post("/clusters")]
async fn create_cluster(
  pool: web::types::State<Pool>,
  web::types::Query(qs): web::types::Query<ClusterQuery>,
  web::types::Json(json): web::types::Json<ClusterPartial>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let nsp = match qs.namespace {
    None => String::from("global"),
    Some(namespace) => namespace,
  };
  let proxy_config = json.proxy_config.to_owned();
  let res =
    repositories::cluster::create_for_namespace(nsp, json, &pool).await?;
  if let Some(proxy_config) = proxy_config {
    repositories::cluster_proxy_config::create_for_cluster(
      res.key.to_owned(),
      proxy_config,
      &pool,
    )
    .await?;
  }
  Ok(web::HttpResponse::Created().json(&res))
}

/// Delete cluster by it's name
#[cfg_attr(feature = "openapi", utoipa::path(
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
))]
#[web::delete("clusters/{name}")]
async fn delete_cluster_by_name(
  pool: web::types::State<Pool>,
  docker_api: web::types::State<bollard::Docker>,
  name: web::types::Path<String>,
  web::types::Query(qs): web::types::Query<ClusterQuery>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let nsp = match qs.namespace {
    None => String::from("global"),
    Some(namespace) => namespace,
  };
  let gen_key = nsp.to_owned() + "-" + &name.into_inner();

  let item =
    repositories::cluster::find_by_key(gen_key.to_owned(), &pool).await?;

  log::info!("deleting cluster cargo");
  repositories::cluster_cargo::get_by_cluster_key(gen_key.to_owned(), &pool)
    .await?;

  repositories::cluster_variable::delete_by_cluster_key(
    gen_key.to_owned(),
    &pool,
  )
  .await?;
  services::cluster::delete_networks(item, &docker_api, &pool).await?;
  repositories::cluster_proxy_config::delete_for_cluster(
    gen_key.to_owned(),
    &pool,
  )
  .await?;
  let res = repositories::cluster::delete_by_key(gen_key, &pool).await?;
  Ok(web::HttpResponse::Ok().json(&res))
}

/// Inspect cluster by it's name
#[cfg_attr(feature = "openapi", utoipa::path(
  get,
  path = "/clusters/{name}/inspect",
  params(
    ("name" = String, path, description = "Name of the cluster"),
    ("namespace" = Option<String>, query, description = "Namespace to add cluster in if empty we use 'default' as value"),
  ),
  responses(
    (status = 200, description = "Cluster information", body = ClusterItemWithRelation),
    (status = 400, description = "Generic database error", body = ApiError),
    (status = 404, description = "id name or namespace name not valid", body = ApiError),
  ),
))]
#[web::get("/clusters/{name}/inspect")]
async fn inspect_cluster_by_name(
  pool: web::types::State<Pool>,
  name: web::types::Path<String>,
  web::types::Query(qs): web::types::Query<ClusterQuery>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let name = name.into_inner();
  let nsp = match qs.namespace {
    None => String::from("global"),
    Some(namespace) => namespace,
  };
  let gen_key = nsp.to_owned() + "-" + &name;
  let item =
    repositories::cluster::find_by_key(gen_key.to_owned(), &pool).await?;
  let networks =
    repositories::cluster_network::list_for_cluster(item, &pool).await?;

  let proxy_config = match repositories::cluster_proxy_config::get_for_cluster(
    gen_key.to_owned(),
    &pool,
  )
  .await
  {
    Err(_) => None,
    Ok(proxy_config) => Some(proxy_config),
  };

  let res = ClusterItemWithRelation {
    name,
    key: gen_key,
    namespace: nsp,
    proxy_config,
    networks: Some(networks),
  };

  Ok(web::HttpResponse::Ok().json(&res))
}

/// Start all cargo inside cluster
#[cfg_attr(feature = "openapi", utoipa::path(
  post,
  path = "/clusters/{name}/start",
  params(
    ("name" = String, path, description = "Name of the cluster"),
    ("namespace" = Option<String>, query, description = "Namespace to add cluster in if empty we use 'global' as value"),
  ),
  responses(
    (status = 200, description = "Cargos have been started"),
    (status = 400, description = "Generic database error", body = ApiError),
    (status = 404, description = "Cluster name of namespace invalid", body = ApiError),
  ),
))]
#[web::post("/clusters/{name}/start")]
async fn start_cluster_by_name(
  name: web::types::Path<String>,
  web::types::Query(qs): web::types::Query<ClusterQuery>,
  pool: web::types::State<Pool>,
  config: web::types::State<DaemonConfig>,
  docker_api: web::types::State<bollard::Docker>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let name = name.into_inner();
  let nsp = match qs.namespace {
    None => String::from("global"),
    Some(namespace) => namespace,
  };
  let gen_key = nsp.to_owned() + "-" + &name;
  let cluster = repositories::cluster::find_by_key(gen_key, &pool).await?;
  services::cluster::start(&cluster, &config, &pool, &docker_api).await?;
  Ok(web::HttpResponse::Ok().into())
}

/// join cargo inside a cluster
#[cfg_attr(feature = "openapi", utoipa::path(
  post,
  path = "/clusters/{name}/join",
  request_body = ClusterJoinBody,
  params(
    ("name" = String, path, description = "Name of the cluster"),
    ("namespace" = Option<String>, query, description = "Namespace to add cluster in if empty we use 'global' as value"),
  ),
  responses(
    (status = 200, description = "Cargo joinned successfully"),
    (status = 400, description = "Generic database error", body = ApiError),
    (status = 404, description = "Cluster name of namespace invalid", body = ApiError),
  ),
))]
#[web::post("/clusters/{name}/join")]
async fn join_cargo_to_cluster(
  pool: web::types::State<Pool>,
  docker_api: web::types::State<bollard::Docker>,
  name: web::types::Path<String>,
  web::types::Query(qs): web::types::Query<ClusterQuery>,
  web::types::Json(payload): web::types::Json<ClusterJoinBody>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let name = name.into_inner();
  let nsp = match qs.namespace {
    None => String::from("global"),
    Some(namespace) => namespace,
  };
  let cluster_key = nsp.to_owned() + "-" + &name;
  let cargo_key = nsp.to_owned() + "-" + &payload.cargo;
  let cluster = repositories::cluster::find_by_key(cluster_key, &pool).await?;
  let cargo = repositories::cargo::find_by_key(cargo_key, &pool).await?;
  let network_key = cluster.key.to_owned() + "-" + &payload.network;
  let network =
    repositories::cluster_network::find_by_key(network_key, &pool).await?;

  log::debug!(
    "joining cargo {:?} into cluster {:?}",
    cargo.key,
    cluster.key
  );
  let join_cargo_opts = JoinCargoOptions {
    cluster,
    cargo,
    network,
  };
  services::cluster::join_cargo(&join_cargo_opts, &docker_api, &pool).await?;
  log::debug!("join success.");
  Ok(web::HttpResponse::Ok().into())
}

/// Count cluster
#[cfg_attr(feature = "openapi", utoipa::path(
  get,
  path = "/clusters/count",
  params(
    ("namespace" = Option<String>, query, description = "Name of the namespace where the cargo is stored"),
  ),
  responses(
    (status = 200, description = "Generic delete", body = PgGenericCount),
    (status = 400, description = "Generic database error", body = ApiError),
    (status = 404, description = "Namespace name not valid", body = ApiError),
  ),
))]
#[web::get("/clusters/count")]
async fn count_cluster(
  pool: web::types::State<Pool>,
  web::types::Query(qs): web::types::Query<ClusterQuery>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let nsp = match qs.namespace {
    None => String::from("global"),
    Some(nsp) => nsp,
  };
  let res = repositories::cluster::count(nsp, &pool).await?;

  Ok(web::HttpResponse::Ok().json(&res))
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
  config.service(start_cluster_by_name);
  config.service(join_cargo_to_cluster);
  config.service(count_cluster);
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
      proxy_config: None,
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
    let srv = generate_server(ntex_config).await;
    test_list(&srv).await?;
    test_list_with_nsp(&srv).await?;
    test_create(&srv).await?;
    test_delete(&srv).await?;
    Ok(())
  }
}
