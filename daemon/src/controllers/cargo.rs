use ntex::web;

use crate::models::Pool;
use super::http_error::HttpError;

#[utoipa::path(
  get,
  path = "/cargos",
  responses(
      (status = 200, description = "Array of cargo", body = CargoItem),
  ),
)]
#[web::get("/cargos")]
pub async fn list(
  pool: web::types::State<Pool>,
) -> Result<web::HttpResponse, HttpError> {
  Ok(
    web::HttpResponse::Ok()
    .content_type("application/json")
    .body("not json lol")
  )
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(list);
}
