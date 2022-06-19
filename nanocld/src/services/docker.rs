use ntex::{web, rt};
use ntex::util::Bytes;
use ntex::channel::mpsc::{self, Receiver};
use ntex::http::StatusCode;
use futures::StreamExt;

use crate::models::GitRepositoryItem;
use crate::controllers::errors::HttpError;

pub async fn build_git_repository(
  docker: web::types::State<bollard::Docker>,
  item: GitRepositoryItem,
) -> Result<Receiver<Result<Bytes, web::error::Error>>, HttpError> {
  let image_name = item.name.to_owned();
  let image_url = item.url + ".git#development";
  let options = bollard::image::BuildImageOptions::<String> {
    dockerfile: String::from("Dockerfile"),
    t: image_name,
    remote: image_url,
    ..Default::default()
  };
  let (tx, rx_body) = mpsc::channel();
  rt::spawn(async move {
    let mut stream = docker.build_image(options, None, None);
    while let Some(result) = stream.next().await {
      println!("docker result {:?}", result);
      match result {
        Err(err) => {
          let err = ntex::web::Error::new(web::error::InternalError::default(
            format!("{:?}", err),
            StatusCode::INTERNAL_SERVER_ERROR,
          ));
          let _ = tx.send(Err::<_, web::error::Error>(err));
        }
        Ok(result) => {
          let data = serde_json::to_string(&result).unwrap();
          let _ = tx.send(Ok::<_, web::error::Error>(Bytes::from(data)));
        }
      }
    }
  });

  Ok(rx_body)
}

pub async fn build_image(
  image_name: String,
  docker: web::types::State<bollard::Docker>,
) -> Result<Receiver<Result<Bytes, web::error::Error>>, HttpError> {
  let (tx, rx_body) = mpsc::channel();
  rt::spawn(async move {
    let mut stream = docker.create_image(
      Some(bollard::image::CreateImageOptions {
        from_image: image_name,
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
          let _ = tx.send(Err::<_, web::error::Error>(err));
        }
        Ok(result) => {
          let data = serde_json::to_string(&result).unwrap();
          let _ = tx.send(Ok::<_, web::error::Error>(Bytes::from(data)));
        }
      }
    }
  });
  Ok(rx_body)
}
