use ntex::web;
use ntex_files as fs;
use utoipa::OpenApi;

use crate::controllers::*;
use crate::models::NamespaceItem;

#[derive(OpenApi)]
#[openapi(handlers(
  namespace::list
), components(NamespaceItem))]
struct ApiDoc;

#[web::get("/explorer/swagger.json")]
async fn get_api_specs() -> Result<web::HttpResponse, web::Error>{
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
