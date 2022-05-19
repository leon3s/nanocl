use ntex::web;
use ntex_files as fs;
use utoipa::OpenApi;

use crate::controllers::namespace::*;
use crate::models::errors;
use crate::models::namespace::Namespace;

#[derive(OpenApi)]
#[openapi(handlers(list_namespace), components(Namespace))]
struct ApiDoc;

#[web::get("/explorer/swagger.json")]
async fn get_api_specs() -> Result<web::HttpResponse, errors::HttpError>{
  let api_spec = ApiDoc::openapi().to_pretty_json().unwrap();
  Ok(
      web::HttpResponse::Ok()
      .content_type("application/json")
      .body(&api_spec)
  )
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(get_api_specs);
  config.service(
    fs::Files::new(
      "/explorer",
      "./daemon/static/swagger")
    .index_file("index.html"),
  );
}
