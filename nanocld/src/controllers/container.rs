use ntex::web;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::errors::HttpResponseError;

#[derive(Serialize, Deserialize)]
pub struct ListContainerQuery {
  cluster: Option<String>,
  cargo: Option<String>,
  namespace: Option<String>,
}

#[web::get("/containers")]
async fn list_containers(
  web::types::Query(qs): web::types::Query<ListContainerQuery>,
  docker_api: web::types::State<bollard::Docker>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let namespace = match qs.namespace {
    None => String::from("global"),
    Some(namespace) => namespace,
  };
  let mut filters = HashMap::new();
  let default_label = format!("namespace={}", &namespace);
  let mut labels = vec![default_label];
  if let Some(ref cluster) = qs.cluster {
    let label = format!("cluster={}-{}", &namespace, &cluster);
    labels.push(label);
  }
  if let Some(ref cargo) = qs.cargo {
    let label = format!("cargo={}-{}", &namespace, &cargo);
    labels.push(label);
  }
  filters.insert(String::from("label"), labels);
  let options = Some(bollard::container::ListContainersOptions::<String> {
    all: true,
    filters,
    ..Default::default()
  });
  let containers = docker_api.list_containers(options).await?;

  Ok(web::HttpResponse::Ok().json(&containers))
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(list_containers);
}
