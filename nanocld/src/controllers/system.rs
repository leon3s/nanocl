use ntex::web;
use serde_json::json;

use crate::errors::HttpResponseError;

use crate::version;

#[web::get("/version")]
async fn get_version() -> Result<web::HttpResponse, HttpResponseError> {
  Ok(web::HttpResponse::Ok().json(&json!({
    "arch": version::ARCH,
    "version": version::VERSION,
    "commit_id": version::COMMIT_ID,
  })))
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(get_version);
}
