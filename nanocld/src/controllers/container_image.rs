use futures::StreamExt;
use ntex::{web, http::StatusCode, channel::mpsc, rt, util::Bytes};

use crate::{errors::HttpResponseError, models::ContainerImagePartial};

#[web::get("/containers/images")]
async fn list_container_image(
  docker_api: web::types::State<bollard::Docker>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let images = docker_api
    .list_images(Some(bollard::image::ListImagesOptions::<String> {
      all: false,
      ..Default::default()
    }))
    .await?;

  Ok(web::HttpResponse::Ok().json(&images))
}

#[web::post("/containers/images")]
async fn create_container_image(
  docker_api: web::types::State<bollard::Docker>,
  web::types::Json(payload): web::types::Json<ContainerImagePartial>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let image_info = payload.name.split(':').collect::<Vec<&str>>();

  if image_info.len() != 2 {
    return Err(HttpResponseError {
      msg: String::from("missing tag in image name"),
      status: StatusCode::BAD_REQUEST,
    });
  }

  let (tx, rx_body) = mpsc::channel();

  log::info!("im called !");
  let from_image = image_info[0].to_string();
  let tag = image_info[1].to_string();
  rt::spawn(async move {
    let mut stream = docker_api.create_image(
      Some(bollard::image::CreateImageOptions {
        from_image,
        tag,
        ..Default::default()
      }),
      None,
      None,
    );

    while let Some(result) = stream.next().await {
      match result {
        Err(err) => {
          let err = ntex::web::Error::new(web::error::InternalError::default(
            format!("{:?}", err),
            StatusCode::INTERNAL_SERVER_ERROR,
          ));
          let result = tx.send(Err::<_, web::error::Error>(err));
          if result.is_err() {
            break;
          }
        }
        Ok(result) => {
          let data = match serde_json::to_string(&result) {
            Err(err) => {
              log::error!("unable to stringify create image info {:#?}", err);
              break;
            }
            Ok(data) => data,
          };
          let result = tx.send(Ok::<_, web::error::Error>(Bytes::from(data)));
          if result.is_err() {
            println!("stop build image");
            break;
          }
        }
      }
    }
  });

  Ok(
    web::HttpResponse::Ok()
      .keep_alive()
      .content_type("nanocl/streaming-v1")
      .streaming(rx_body),
  )
}

#[web::delete("/containers/images/{id_or_name}")]
async fn delete_container_image_by_name(
  docker_api: web::types::State<bollard::Docker>,
  id_or_name: web::types::Path<String>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let id_or_name = id_or_name.into_inner();
  docker_api.remove_image(&id_or_name, None, None).await?;
  Ok(web::HttpResponse::Ok().into())
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(list_container_image);
  config.service(create_container_image);
  config.service(delete_container_image_by_name);
}
