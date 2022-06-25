use std::collections::HashMap;

use ntex::{web, rt};
use ntex::util::Bytes;
use ntex::channel::mpsc::{self, Receiver};
use ntex::http::StatusCode;
use futures::StreamExt;

use crate::models::{GitRepositoryItem, GitRepositoryBranchItem};
use crate::controllers::errors::HttpError;

pub async fn build_git_repository(
  image_name: String,
  item: GitRepositoryItem,
  branch: GitRepositoryBranchItem,
  docker_api: web::types::State<bollard::Docker>,
) -> Result<Receiver<Result<Bytes, web::error::Error>>, HttpError> {
  let image_url = item.url + ".git#" + &branch.name;
  let mut labels: HashMap<String, String> = HashMap::new();
  labels.insert(String::from("commit"), branch.last_commit_sha);
  let options = bollard::image::BuildImageOptions::<String> {
    dockerfile: String::from("Dockerfile"),
    t: image_name,
    labels,
    remote: image_url,
    ..Default::default()
  };
  let (tx, rx_body) = mpsc::channel();

  rt::spawn(async move {
    let mut stream = docker_api.build_image(options, None, None);
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

#[allow(dead_code)]
pub async fn build_image(
  image_name: String,
  docker_api: web::types::State<bollard::Docker>,
) -> Result<Receiver<Result<Bytes, web::error::Error>>, HttpError> {
  let (tx, rx_body) = mpsc::channel();
  rt::spawn(async move {
    let mut stream = docker_api.create_image(
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
