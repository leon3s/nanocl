use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::nanocld::{cargo::CargoProxyConfigPartial, cluster::ClusterJoinPartial};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Cargo {
  pub(crate) name: String,
  #[serde(rename(deserialize = "image"))]
  pub(crate) image_name: String,
  pub(crate) proxy_config: Option<CargoProxyConfigPartial>,
  #[serde(rename(deserialize = "envs"))]
  pub(crate) environnements: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Network {
  pub(crate) name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GitRepository {
  pub(crate) name: String,
  pub(crate) url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Cluster {
  pub(crate) name: String,
  pub(crate) auto_start: Option<bool>,
  #[serde(rename(deserialize = "vars"))]
  pub(crate) variables: Option<HashMap<String, String>>,
  pub(crate) joins: Option<Vec<ClusterJoinPartial>>,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub(crate) struct CargoProxyConf {
//   target_port: u32,
//   host_ip: String,
//   domain_name: String,
// }

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct NamespaceConfig {
  // name of the namespace
  pub(crate) name: String,
  // list of cargo to deploy
  pub(crate) cargoes: Vec<Cargo>,
  // list of network to create when deploy
  pub(crate) networks: Vec<Network>,
  // List of configuration a bit like github workflow matrix
  pub(crate) clusters: Vec<Cluster>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum YmlConfigTypes {
  #[serde(rename(deserialize = "namespace"))]
  Namespace,
  #[serde(rename(deserialize = "cargo"))]
  Cargo,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct YmlFile {
  #[serde(rename(deserialize = "type"))]
  pub(crate) file_type: YmlConfigTypes,
}
