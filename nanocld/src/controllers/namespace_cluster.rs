use ntex::web;

use crate::models::Pool;

use crate::utils::get_poll_conn;
use crate::repositories::cluster;
use crate::repositories::errors::db_bloking_error;

use super::errors::HttpError;

#[utoipa::path(
  get,
  path = "/namespaces/{name}/clusters",
  params(
    ("name" = String, path, description = "Id or Name of the namespace"),
  ),
  responses(
    (status = 201, description = "Fresh created cluster", body = ClusterItem),
    (status = 400, description = "Generic database error"),
    (status = 404, description = "Namespace name not valid"),
  ),
)]
#[web::get("/namespaces/{name}/clusters")]
async fn list(
    poll: web::types::State<Pool>,
    name: web::types::Path<String>,
) -> Result<web::HttpResponse, HttpError> {
    let nsp = name.into_inner();
    let conn = get_poll_conn(poll)?;

    let res = web::block(move || cluster::find_by_namespace(nsp, &conn)).await;

    match res {
        Err(err) => Err(db_bloking_error(err)),
        Ok(items) => Ok(web::HttpResponse::Ok().json(&items)),
    }
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
    config.service(list);
}

#[cfg(test)]
mod test_namespace_cluster {
    use ntex::web::test::TestServer;

    use crate::utils::test::*;
    use super::ntex_config;

    async fn test_list(srv: &TestServer) {
        let resp = srv
            .get("/namespaces/default/clusters")
            .send()
            .await
            .unwrap();
        println!("{:?}", resp);
        assert!(resp.status().is_success());
    }

    #[ntex::test]
    async fn main() {
        let srv = generate_server(ntex_config);
        test_list(&srv).await;
    }
}
