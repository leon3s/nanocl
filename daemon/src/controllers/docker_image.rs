use ntex::web;

use crate::{
  app_state::DaemonState,
  responses::errors,
};

#[web::get("/docker/images")]
pub async fn get_docker_image(
  state: web::types::State<DaemonState>,
) -> Result<web::HttpResponse, errors::HttpError> {
  let docker = &state.docker_api;
  let images = match docker.images().list(&Default::default()).await {
    Ok(images) => images,
    Err(err) => {
      return Err(errors::docker_error(err));
    }
  };
  Ok(
    web::HttpResponse::Ok()
    .content_type("application/json")
    .json(&images)
  )
}

pub fn ctrl_config(config: &mut web::ServiceConfig) {
  config.service(get_docker_image);
}
