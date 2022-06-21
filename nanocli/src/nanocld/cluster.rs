use clap::Parser;
use tabled::Tabled;
use serde::{Serialize, Deserialize};

use super::{client::Nanocld, error::Error};

#[derive(Tabled, Serialize, Deserialize)]
pub struct ClusterItem {
  pub(crate) key: String,
  pub(crate) namespace: String,
  pub(crate) name: String,
}

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct ClusterPartial {
  pub name: String,
}

#[derive(Debug, Tabled, Serialize, Deserialize)]
pub struct ClusterNetworkItem {
  pub(crate) key: String,
  pub(crate) name: String,
  pub(crate) docker_network_id: String,
  pub(crate) cluster_key: String,
}

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct ClusterNetworkPartial {
  pub(crate) name: String,
}

impl Nanocld {
  pub async fn list_cluster(&self) -> Result<Vec<ClusterItem>, Error> {
    let mut res = self
      .get(String::from("/clusters"))
      .send()
      .await
      .map_err(Error::SendRequest)?;
    let items = res
      .json::<Vec<ClusterItem>>()
      .await
      .map_err(Error::JsonPayload)?;
    Ok(items)
  }

  pub async fn create_cluster(
    &self,
    item: &ClusterPartial,
  ) -> Result<ClusterItem, Error> {
    let mut res = self
      .post(String::from("/clusters"))
      .send_json(&item)
      .await
      .map_err(Error::SendRequest)?;
    let item = res
      .json::<ClusterItem>()
      .await
      .map_err(Error::JsonPayload)?;
    Ok(item)
  }

  pub async fn delete_cluster(&self, name: String) -> Result<(), Error> {
    let _res = self
      .delete(format!("/clusters/{name}", name = name))
      .send()
      .await
      .map_err(Error::SendRequest)?;
    Ok(())
  }

  pub async fn list_cluster_network(
    &self,
    cluster_name: String,
  ) -> Result<Vec<ClusterNetworkItem>, Error> {
    let mut res = self
      .get(format!("/clusters/{name}/networks", name = cluster_name))
      .send()
      .await
      .map_err(Error::SendRequest)?;
    let items = res
      .json::<Vec<ClusterNetworkItem>>()
      .await
      .map_err(Error::JsonPayload)?;
    Ok(items)
  }

  pub async fn create_cluster_network(
    &self,
    cluster_name: String,
    item: &ClusterNetworkPartial,
  ) -> Result<ClusterNetworkItem, Error> {
    let mut res = self
      .post(format!("/clusters/{name}/networks", name = cluster_name))
      .send_json(item)
      .await
      .map_err(Error::SendRequest)?;
    let item = res
      .json::<ClusterNetworkItem>()
      .await
      .map_err(Error::JsonPayload)?;
    Ok(item)
  }

  pub async fn delete_cluster_network(
    &self,
    cluster_name: String,
    network_name: String,
  ) -> Result<(), Error> {
    let _res = self
      .delete(format!(
        "/clusters/{c_name}/networks/{n_name}",
        c_name = cluster_name,
        n_name = network_name
      ))
      .send()
      .await
      .map_err(Error::SendRequest)?;
    Ok(())
  }
}
