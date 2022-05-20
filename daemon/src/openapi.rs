use ntex::web;
use ntex_files as fs;
use utoipa::OpenApi;

use crate::models::*;
use crate::controllers::*;

#[derive(OpenApi)]
#[openapi(handlers(
  namespace::list,
  namespace::create
), components(
  NamespaceItem,
  NamespaceCreate
))]
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
      "./static/swagger")
    .index_file("index.html"),
  );
}
