use ntex::web;

use ntex::http::StatusCode;

use crate::repositories;
use crate::models::{Pool, NamespaceCreate};

use super::utils::get_poll;
use super::http_error::{HttpError, db_bloking_error};

#[utoipa::path(
  get,
  path = "/namespaces",
  responses(
      (status = 200, description = "Array of namespace found", body = NamespaceItem),
  ),
)]
pub async fn list() {

}

#[utoipa::path(
  post,
  path = "/namespaces",
  request_body = NamespaceCreate,
  responses(
    (status = 200, description = "Fresh created namespace", body = NamespaceItem),
    (status = 400, description = "Generic database error"),
    (status = 422, description = "The provided payload is not valid"),
  ),
)]
#[web::post("/namespaces")]
pub async fn create(
  payload: web::types::Json<NamespaceCreate>,
  pool: web::types::State<Pool>,
) -> Result<web::HttpResponse, HttpError> {
  let new_namespace = payload.into_inner();
  let conn = get_poll(pool)?;

  let res = web::block(move || {
    repositories::namespace::create(new_namespace, &conn)
  }).await;

  match res {
    Ok(inserted_namespace) => {
      Ok(
        web::HttpResponse::Created()
        .json(&inserted_namespace)
      )
    },
    Err(err) => {
      Err(db_bloking_error(err))
    }
  }
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(create);
}
