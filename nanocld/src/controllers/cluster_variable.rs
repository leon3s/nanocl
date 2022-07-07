use ntex::web;
use serde::{Serialize, Deserialize};

use crate::repositories;
use crate::models::{Pool, ClusterVariablePartial};

use super::utils::gen_nsp_key_by_name;

use crate::errors::HttpResponseError;

#[derive(Serialize, Deserialize)]
pub struct ClusterVaribleQuery {
  namespace: Option<String>,
}

/// Create cluster variable
#[utoipa::path(
  post,
  path = "/clusters/{c_name}/variables",
  request_body = ClusterVariablePartial,
  params(
    ("c_name" = String, path, description = "name of the cluster"),
    ("namespace" = Option<String>, query, description = "Name of the namespace where the cluster is stored is empty we use 'global' as value"),
  ),
  responses(
    (status = 200, description = "Fresh cluster variable", body = ClusterVariableItem),
    (status = 400, description = "Generic database error", body = ApiError),
    (status = 404, description = "Cluster name or Namespace not valid", body = ApiError),
  ),
)]
#[web::post("/clusters/{c_name}/variables")]
async fn create_cluster_variable(
  pool: web::types::State<Pool>,
  c_name: web::types::Path<String>,
  web::types::Query(qs): web::types::Query<ClusterVaribleQuery>,
  web::types::Json(payload): web::types::Json<ClusterVariablePartial>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let name = c_name.into_inner();
  let cluster_key = gen_nsp_key_by_name(&qs.namespace, &name);

  repositories::cluster::find_by_key(cluster_key.to_owned(), &pool).await?;
  let cluster_var = repositories::cluster_variable::create(
    cluster_key.to_owned(),
    payload,
    &pool,
  )
  .await?;

  Ok(web::HttpResponse::Created().json(&cluster_var))
}

/// List variable of a cluster
#[utoipa::path(
  get,
  path = "/clusters/{c_name}/variables",
  params(
    ("c_name" = String, path, description = "name of the cluster"),
    ("namespace" = Option<String>, query, description = "Name of the namespace where the cluster is stored is empty we use 'global' as value"),
  ),
  responses(
    (status = 200, description = "List of variable for given cluster", body = [ClusterVariableItem]),
    (status = 400, description = "Generic database error", body = ApiError),
    (status = 404, description = "Cluster name or Namespace not valid", body = ApiError),
  ),
)]
#[web::get("/clusters/{c_name}/variables")]
async fn list_cluster_variable(
  pool: web::types::State<Pool>,
  c_name: web::types::Path<String>,
  web::types::Query(qs): web::types::Query<ClusterVaribleQuery>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let name = c_name.into_inner();

  let cluster_key = gen_nsp_key_by_name(&qs.namespace, &name);

  let cluster_variables = repositories::cluster_variable::list_by_cluster(
    cluster_key.to_owned(),
    &pool,
  )
  .await?;

  Ok(web::HttpResponse::Ok().json(&cluster_variables))
}

#[derive(Serialize, Deserialize)]
pub struct ClusterVariablePath {
  c_name: String,
  v_name: String,
}

/// Delete cluster variable by it's name
#[utoipa::path(
  delete,
  path = "/clusters/{c_name}/variables/{v_name}",
  params(
    ("c_name" = String, path, description = "name of the cluster"),
    ("v_name" = String, path, description = "name of the variable"),
    ("namespace" = Option<String>, query, description = "Name of the namespace where the cluster is stored is empty we use 'global' as value"),
  ),
  responses(
    (status = 200, description = "Generic delete response", body = PgDeleteGeneric),
    (status = 400, description = "Generic database error", body = ApiError),
    (status = 404, description = "Cluster name or Namespace not valid", body = ApiError),
  ),
)]
#[web::delete("/clusters/{c_name}/variables/{v_name}")]
async fn delete_cluster_variable(
  pool: web::types::State<Pool>,
  url_path: web::types::Path<ClusterVariablePath>,
  web::types::Query(qs): web::types::Query<ClusterVaribleQuery>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let var_name = format!("{}-{}", &url_path.c_name, &url_path.v_name);
  let var_key = gen_nsp_key_by_name(&qs.namespace, &var_name);

  let res =
    repositories::cluster_variable::delete_by_key(var_key, &pool).await?;
  Ok(web::HttpResponse::Ok().json(&res))
}

/// Get cluster variable by it's name
#[utoipa::path(
  get,
  path = "/clusters/{c_name}/variables/{v_name}",
  params(
    ("c_name" = String, path, description = "name of the cluster"),
    ("v_name" = String, path, description = "name of the variable"),
    ("namespace" = Option<String>, query, description = "Name of the namespace where the cluster is stored is empty we use 'global' as value"),
  ),
  responses(
    (status = 200, description = "Generic delete response", body = PgDeleteGeneric),
    (status = 400, description = "Generic database error", body = ApiError),
    (status = 404, description = "Cluster name or Namespace not valid", body = ApiError),
  ),
)]
#[web::get("/clusters/{c_name}/variables/{v_name}")]
async fn get_cluster_variable_by_name(
  pool: web::types::State<Pool>,
  url_path: web::types::Path<ClusterVariablePath>,
  web::types::Query(qs): web::types::Query<ClusterVaribleQuery>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let var_name = format!("{}-{}", &url_path.c_name, &url_path.v_name);
  let var_key = gen_nsp_key_by_name(&qs.namespace, &var_name);

  let res = repositories::cluster_variable::find_by_key(var_key, &pool).await?;
  Ok(web::HttpResponse::Ok().json(&res))
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(create_cluster_variable);
  config.service(list_cluster_variable);
  config.service(delete_cluster_variable);
  config.service(get_cluster_variable_by_name);
}
