use ntex::web;
use ntex::http::StatusCode;
use std::collections::HashMap;

use crate::models::CargoItem;

use crate::controllers::errors::HttpError;
use crate::services::errors::docker_error;

pub async fn create_containers(
  item: &CargoItem,
  network_key: String,
  labels: Option<&mut HashMap<String, String>>,
  docker_api: &web::types::State<bollard::Docker>,
) -> Result<Vec<String>, HttpError> {
  log::debug!(
    "creating containers for cargo {:?} with labels {:?}",
    &item,
    &labels
  );
  let mut container_ids: Vec<String> = Vec::new();
  let image_name = item.image_name.clone();
  if docker_api.inspect_image(&item.image_name).await.is_err() {
    return Err(HttpError {
      msg: String::from("image name is not valid"),
      status: StatusCode::BAD_REQUEST,
    });
  }
  log::debug!("image name not empty {:?}", &image_name);
  let image = Some(image_name.to_owned());
  let mut labels: HashMap<String, String> = match labels {
    None => HashMap::new(),
    Some(labels) => labels.to_owned(),
  };
  labels.insert(String::from("namespace"), item.namespace_name.to_owned());
  labels.insert(String::from("cargo"), item.key.to_owned());
  let config = bollard::container::Config {
    image,
    tty: Some(true),
    labels: Some(labels),
    attach_stdout: Some(true),
    attach_stderr: Some(true),
    host_config: Some(bollard::models::HostConfig {
      network_mode: Some(network_key),
      ..Default::default()
    }),
    ..Default::default()
  };
  let res = match docker_api
    .create_container(
      None::<bollard::container::CreateContainerOptions<String>>,
      config,
    )
    .await
  {
    Err(err) => return Err(docker_error(err)),
    Ok(res) => res,
  };
  container_ids.push(res.id);
  Ok(container_ids)
}

pub async fn list_containers(
  cargo_key: String,
  docker_api: &web::types::State<bollard::Docker>,
) -> Result<Vec<bollard::models::ContainerSummary>, HttpError> {
  let target_cluster = &format!("cargo={}", &cargo_key);
  let mut filters = HashMap::new();
  filters.insert("label", vec![target_cluster.as_str()]);
  let options = Some(bollard::container::ListContainersOptions {
    all: true,
    filters,
    ..Default::default()
  });
  let containers = docker_api
    .list_containers(options)
    .await
    .map_err(docker_error)?;
  Ok(containers)
}
