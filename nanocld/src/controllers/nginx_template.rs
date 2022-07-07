use ntex::web;

use crate::repositories;
use crate::models::{Pool, NginxTemplateItem};

use crate::errors::HttpResponseError;

/// List all nginx template
#[utoipa::path(
  get,
  path = "/nginx_templates",
  responses(
      (status = 200, description = "Array of nginx templates", body = [NginxTemplateItem]),
  ),
)]
#[web::get("/nginx_templates")]
async fn list_nginx_template(
  pool: web::types::State<Pool>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let items = repositories::nginx_template::list(&pool).await?;

  Ok(web::HttpResponse::Ok().json(&items))
}

/// Create nginx template
#[utoipa::path(
  post,
  path = "/nginx_templates",
  responses(
    (status = 201, description = "Nginx template created", body = NginxTemplateItem)
  )
)]
#[web::post("/nginx_templates")]
async fn create_nginx_template(
  pool: web::types::State<Pool>,
  web::types::Json(payload): web::types::Json<NginxTemplateItem>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let res = repositories::nginx_template::create(payload, &pool).await?;

  Ok(web::HttpResponse::Created().json(&res))
}

/// Delete nginx template by name
#[web::delete("/nginx_templates/{name}")]
async fn delete_nginx_template_by_name(
  pool: web::types::State<Pool>,
  name: web::types::Path<String>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let res =
    repositories::nginx_template::delete_by_name(name.into_inner(), &pool)
      .await?;

  Ok(web::HttpResponse::Ok().json(&res))
}

/// Inspect nginx template by name
#[web::get("/nginx_templates/{name}")]
async fn inspect_nginx_template_by_name(
  pool: web::types::State<Pool>,
  name: web::types::Path<String>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let res =
    repositories::nginx_template::get_by_name(name.into_inner(), &pool).await?;

  Ok(web::HttpResponse::Ok().json(&res))
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(list_nginx_template);
  config.service(create_nginx_template);
  config.service(delete_nginx_template_by_name);
  config.service(inspect_nginx_template_by_name);
}
