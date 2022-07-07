use ntex::{web, http::StatusCode};

use crate::errors::HttpResponseError;

#[web::get("/containers/images")]
async fn list_image(
  docker_api: web::types::State<bollard::Docker>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let images = docker_api
    .list_images(None::<bollard::image::ListImagesOptions<String>>)
    .await
    .map_err(|err| HttpResponseError {
      msg: format!("unable to list image {}", err),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

  Ok(web::HttpResponse::Ok().json(&images))
}

#[web::post("/containers/images")]
async fn build_image(
  docker_api: web::types::State<bollard::Docker>,
) -> Result<web::HttpResponse, HttpResponseError> {
  Ok(web::HttpResponse::Ok().into())
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(list_image);
}
