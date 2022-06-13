use ntex::web;

use crate::models::{NamespaceCreate, Pool};
use crate::repositories::namespace;

use super::errors::HttpError;

/// List all namespaces
#[utoipa::path(
  get,
  path = "/namespaces",
  responses(
      (status = 200, description = "Array of namespace", body = [NamespaceItem]),
  ),
)]
#[web::get("/namespaces")]
async fn list(
  pool: web::types::State<Pool>,
) -> Result<web::HttpResponse, HttpError> {
  let items = namespace::list(&pool).await?;

  Ok(web::HttpResponse::Ok().json(&items))
}

/// Inspect namespace by id or name
#[utoipa::path(
  get,
  path = "/namespaces/{id}/inspect",
  responses(
      (status = 200, description = "Namespace found", body = NamespaceItem),
      (status = 404, description = "Namespace not found"),
  ),
  params(
    ("id" = String, path, description = "id or name of the namespace"),
  )
)]
#[web::get("/namespaces/{id}/inspect")]
async fn get_by_id_or_name(
  id: web::types::Path<String>,
  pool: web::types::State<Pool>,
) -> Result<web::HttpResponse, HttpError> {
  let id_or_name = id.into_inner();
  let item = namespace::inspect_by_id_or_name(id_or_name, &pool).await?;

  Ok(web::HttpResponse::Ok().json(&item))
}

/// Create a new namespace
#[utoipa::path(
  post,
  path = "/namespaces",
  request_body = NamespaceCreate,
  responses(
    (status = 201, description = "Fresh created namespace", body = NamespaceItem),
    (status = 400, description = "Generic database error"),
    (status = 422, description = "The provided payload is not valid"),
  ),
)]
#[web::post("/namespaces")]
async fn create(
  pool: web::types::State<Pool>,
  payload: web::types::Json<NamespaceCreate>,
) -> Result<web::HttpResponse, HttpError> {
  let new_namespace = payload.into_inner();
  let item = namespace::create(new_namespace, &pool).await?;

  Ok(web::HttpResponse::Created().json(&item))
}

/// Delete a namespace
#[utoipa::path(
    delete,
    path = "/namespaces/{id}",
    responses(
        (status = 200, description = "database generic delete response", body = PgDeleteGeneric),
    ),
    params(
        ("id" = String, path, description = "id or name of the namespace"),
    )
)]
#[web::delete("/namespaces/{id}")]
async fn delete_by_id_or_name(
  id: web::types::Path<String>,
  pool: web::types::State<Pool>,
) -> Result<web::HttpResponse, HttpError> {
  let id_or_name = id.into_inner();
  let res = namespace::delete_by_id_or_name(id_or_name, &pool).await?;
  Ok(web::HttpResponse::Ok().json(&res))
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(list);
  config.service(create);
  config.service(get_by_id_or_name);
  config.service(delete_by_id_or_name);
}

#[cfg(test)]
mod test_namespace {
  use serde_json::json;

  use crate::models::{NamespaceCreate, PgDeleteGeneric};
  use crate::utils::test::*;

  use super::ntex_config;

  async fn test_list(srv: &TestServer) -> TestReturn {
    let resp = srv.get("/namespaces").send().await?;

    assert!(resp.status().is_success());
    Ok(())
  }

  async fn test_create(srv: &TestServer) -> TestReturn {
    let new_namespace = NamespaceCreate {
      name: String::from("default"),
    };

    let resp = srv.post("/namespaces").send_json(&new_namespace).await?;

    assert!(resp.status().is_success());
    Ok(())
  }

  async fn test_fail_create(srv: &TestServer) -> TestReturn {
    let resp = srv
      .post("/namespaces")
      .send_json(&json!({
          "name": 1,
      }))
      .await?;

    assert!(resp.status().is_client_error());

    let resp = srv.post("/namespaces").send().await?;

    assert!(resp.status().is_client_error());
    Ok(())
  }

  async fn test_inspect_by_id(srv: &TestServer) -> TestReturn {
    let resp = srv
      .get(format!("/namespaces/{name}/inspect", name = "default"))
      .send()
      .await?;

    assert!(resp.status().is_success());
    Ok(())
  }

  async fn test_delete(srv: &TestServer) -> TestReturn {
    let mut resp = srv
      .delete(format!("/namespaces/{name}", name = "default"))
      .send()
      .await?;

    let body = resp.json::<PgDeleteGeneric>().await?;
    assert_eq!(body.count, 1);
    assert!(resp.status().is_success());
    Ok(())
  }

  #[ntex::test]
  async fn main() -> TestReturn {
    let srv = generate_server(ntex_config);

    test_fail_create(&srv).await?;
    test_create(&srv).await?;
    test_inspect_by_id(&srv).await?;
    test_list(&srv).await?;
    test_delete(&srv).await?;
    Ok(())
  }
}
