use ntex::web;
use serde::{Serialize, Deserialize};

use crate::repositories::cluster;
use crate::models::{Pool, ClusterCreate};

use super::errors::HttpError;

#[derive(Debug, Serialize, Deserialize)]
struct ClusterQuery {
  pub(crate) namespace: Option<String>,
}

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
async fn list(
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

#[utoipa::path(
  post,
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
async fn create(
  pool: web::types::State<Pool>,
  web::types::Json(json): web::types::Json<ClusterCreate>,
  web::types::Query(qs): web::types::Query<ClusterQuery>,
) -> Result<web::HttpResponse, HttpError> {
  Ok(web::HttpResponse::Ok().into())
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
    config.service(list);
    config.service(create);
}

#[cfg(test)]
mod test_namespace_cluster {
    use crate::utils::test::*;

    use super::*;

    async fn test_list(srv: &TestServer) -> TestReturn {
        let resp = srv
        .get("/clusters")
        .send()
        .await?;

        assert!(resp.status().is_success());
        Ok(())
    }

    async fn test_list_with_nsp(srv: &TestServer) -> TestReturn {
      let resp = srv
      .get("/clusters")
      .query(&ClusterQuery {
        namespace: Some(String::from("test"))
      })?.send().await?;

      assert!(resp.status().is_success());
      Ok(())
    }

    #[ntex::test]
    async fn main() -> TestReturn {
        let srv = generate_server(ntex_config);
        test_list(&srv).await?;
        test_list_with_nsp(&srv).await?;
        Ok(())
    }
}
