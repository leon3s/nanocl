use ntex::web;
use serde::{Serialize, Deserialize};

use super::errors::HttpError;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNetworkQuery {
  pub(crate) cluster: String,
}

#[web::get("/clusters/{id}/networks")]
async fn list(
  cluster_id: web::types::Path<String>,
  web::types::Query(qs): web::types::Query<ClusterNetworkQuery>,
) -> Result<web::HttpResponse, HttpError> {
  // qs.cluster
  Ok(web::HttpResponse::Ok().into())
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(list);
}
