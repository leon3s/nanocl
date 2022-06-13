use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct YmlFile {
  // name of the namespace
  pub(crate) name: String,
  // list of cargo to deploy
  pub(crate) cargos: Vec<Cargo>,
  // list of network to create when deploy
  pub(crate) networks: Vec<Network>,
  // List of configuration a bit like github workflow matrix
  pub(crate) clusters: Vec<Cluster>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Cargo {
  pub(crate) name: String,
  pub(crate) image: Option<String>,
  pub(crate) git_repository: Option<String>,
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
  pub(crate) env: Option<HashMap<String, String>>,
}
