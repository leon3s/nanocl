use ntex::web;

use crate::models::Pool;

use crate::repositories::nginx_template;

use super::errors::HttpError;

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
) -> Result<web::HttpResponse, HttpError> {
  let items = nginx_template::list(&pool).await?;
  Ok(web::HttpResponse::Ok().json(&items))
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(list_nginx_template);
}
