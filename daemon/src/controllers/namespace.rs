/**
 * HTTP Method to administrate namespaces
 */
use ntex::web;
use crate::repositories;
use crate::models::{
  Pool,
  NamespaceCreate,
};

use super::utils::get_poll_conn;
use super::http_error::{HttpError, db_bloking_error};

#[utoipa::path(
  get,
  path = "/namespaces",
  responses(
      (status = 200, description = "Array of namespace", body = NamespaceItem),
  ),
)]
#[web::get("/namespaces")]
pub async fn list(
  pool: web::types::State<Pool>,
) -> Result<web::HttpResponse, HttpError> {
  let conn = get_poll_conn(pool)?;

  let res = web::block(move || {
    repositories::namespace::find_all(&conn)
  }).await;

  match res {
    Err(err) => {
      Err(db_bloking_error(err))
    },
    Ok(namespaces) => {
      Ok(
        web::HttpResponse::Ok()
        .json(&namespaces)
      )
    },
  }
}

#[utoipa::path(
  get,
  path = "/namespaces/{id_or_name}",
  responses(
      (status = 200, description = "Namespace found", body = NamespaceItem),
      (status = 404, description = "Namespace not found"),
  ),
  params(
    ("id_or_name" = String, path, description = "Id or Name of the namespace"),
  )
)]
#[web::get("/namespaces/{id_or_name}")]
pub async fn get_by_id_or_name(
  id_or_name: web::types::Path<String>,
  pool: web::types::State<Pool>,
) -> Result<web::HttpResponse, HttpError> {
  let conn = get_poll_conn(pool)?;

  let res = web::block(move || {
    repositories::namespace::find_by_id_or_name(id_or_name.to_owned(), &conn)
  }).await;

  match res {
    Err(err) => {
      eprintln!("error : {:?}", err);
      Err(db_bloking_error(err))
    },
    Ok(namespace) => {
      Ok(
        web::HttpResponse::Ok()
        .json(&namespace)
      )
    }
  }
}

#[utoipa::path(
  delete,
  path = "/namespaces/{id_or_name}",
  responses(
      (status = 200, description = "Database delete response", body = PgDeleteGeneric),
  ),
  params(
    ("id_or_name" = String, path, description = "Id or Name of the namespace"),
  )
)]
#[web::delete("/namespaces/{id_or_name}")]
pub async fn delete_by_id_or_name(
  id_or_name: web::types::Path<String>,
  pool: web::types::State<Pool>,
) -> Result<web::HttpResponse, HttpError> {
  let conn = get_poll_conn(pool)?;

  let res = web::block(move || {
    repositories::namespace::delete_by_id_or_name(id_or_name.to_owned(), &conn)
  }).await;
  match res {
    Err(err) => {
      Err(db_bloking_error(err))
    },
    Ok(json) => {
      Ok(
        web::HttpResponse::Ok()
        .json(&json)
      )
    }
  }
}

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
pub async fn create(
  payload: web::types::Json<NamespaceCreate>,
  pool: web::types::State<Pool>,
) -> Result<web::HttpResponse, HttpError> {
  let new_namespace = payload.into_inner();
  let conn = get_poll_conn(pool)?;

  let res = web::block(move || {
    repositories::namespace::create(new_namespace, &conn)
  }).await;

  match res {
    Err(err) => {
      Err(db_bloking_error(err))
    },
    Ok(inserted_namespace) => {
      Ok(
        web::HttpResponse::Created()
        .json(&inserted_namespace)
      )
    },
  }
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(list);
  config.service(create);
  config.service(get_by_id_or_name);
  config.service(delete_by_id_or_name);
}
