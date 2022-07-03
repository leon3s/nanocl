use clap::Parser;
use tabled::Tabled;
use serde::{Serialize, Deserialize};

use super::{
  client::Nanocld,
  error::{NanocldError, is_api_error},
  models::PgGenericCount,
};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterVarPartial {
  pub(crate) name: String,
  pub(crate) value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterJoinPartial {
  pub(crate) network: String,
  pub(crate) cargo: String,
}

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct ClusterNetworkPartial {
  pub(crate) name: String,
}

impl Nanocld {
  pub async fn list_cluster(&self) -> Result<Vec<ClusterItem>, NanocldError> {
    let mut res = self.get(String::from("/clusters")).send().await?;
    let status = res.status();
    is_api_error(&mut res, &status).await?;
    let items = res.json::<Vec<ClusterItem>>().await?;

    Ok(items)
  }

  pub async fn create_cluster(
    &self,
    item: &ClusterPartial,
  ) -> Result<ClusterItem, NanocldError> {
    let mut res = self
      .post(String::from("/clusters"))
      .send_json(&item)
      .await?;
    let status = res.status();
    is_api_error(&mut res, &status).await?;
    let item = res.json::<ClusterItem>().await?;

    Ok(item)
  }

  pub async fn delete_cluster(&self, name: String) -> Result<(), NanocldError> {
    let mut res = self
      .delete(format!("/clusters/{name}", name = name))
      .send()
      .await?;
    let status = res.status();
    is_api_error(&mut res, &status).await?;

    Ok(())
  }

  pub async fn list_cluster_network(
    &self,
    cluster_name: String,
  ) -> Result<Vec<ClusterNetworkItem>, NanocldError> {
    let mut res = self
      .get(format!("/clusters/{name}/networks", name = cluster_name))
      .send()
      .await?;
    let status = res.status();
    is_api_error(&mut res, &status).await?;
    let items = res.json::<Vec<ClusterNetworkItem>>().await?;

    Ok(items)
  }

  pub async fn create_cluster_network(
    &self,
    cluster_name: String,
    item: &ClusterNetworkPartial,
  ) -> Result<ClusterNetworkItem, NanocldError> {
    let mut res = self
      .post(format!("/clusters/{name}/networks", name = cluster_name))
      .send_json(item)
      .await
      .map_err(NanocldError::SendRequest)?;
    let status = res.status();
    is_api_error(&mut res, &status).await?;
    let item = res.json::<ClusterNetworkItem>().await?;

    Ok(item)
  }

  pub async fn delete_cluster_network(
    &self,
    cluster_name: String,
    network_name: String,
  ) -> Result<(), NanocldError> {
    let mut res = self
      .delete(format!(
        "/clusters/{c_name}/networks/{n_name}",
        c_name = cluster_name,
        n_name = network_name
      ))
      .send()
      .await?;
    let status = res.status();
    is_api_error(&mut res, &status).await?;

    Ok(())
  }

  pub async fn create_cluster_var(
    &self,
    c_name: &str,
    item: ClusterVarPartial,
  ) -> Result<(), NanocldError> {
    let mut res = self
      .post(format!("/clusters/{c_name}/variables", c_name = c_name))
      .send_json(&item)
      .await?;
    let status = res.status();
    is_api_error(&mut res, &status).await?;

    Ok(())
  }

  // Todo be edit and delete cluster vars
  pub async fn _delete_cluster_var(
    &self,
    c_name: &str,
    v_name: &str,
  ) -> Result<(), NanocldError> {
    let mut res = self
      .delete(format!(
        "/clusters/{c_name}/variables/{v_name}",
        c_name = c_name,
        v_name = v_name
      ))
      .send()
      .await?;
    let status = res.status();
    is_api_error(&mut res, &status).await?;

    Ok(())
  }

  pub async fn join_cluster_cargo(
    &self,
    c_name: &str,
    item: &ClusterJoinPartial,
  ) -> Result<(), NanocldError> {
    let mut res = self
      .post(format!("/clusters/{c_name}/join", c_name = c_name))
      .send_json(item)
      .await?;
    let status = res.status();
    is_api_error(&mut res, &status).await?;

    Ok(())
  }

  pub async fn start_cluster(&self, c_name: &str) -> Result<(), NanocldError> {
    let mut res = self
      .post(format!("/clusters/{c_name}/start", c_name = c_name))
      .send()
      .await?;
    let status = res.status();
    is_api_error(&mut res, &status).await?;

    Ok(())
  }

  pub async fn count_cluster(
    &self,
    _namespace: &str, // Todo add query
  ) -> Result<PgGenericCount, NanocldError> {
    let mut res = self.get(String::from("/clusters/count")).send().await?;
    let status = res.status();
    is_api_error(&mut res, &status).await?;
    let count = res.json::<PgGenericCount>().await?;
    Ok(count)
  }

  pub async fn count_cluster_network_by_nsp(
    &self,
    _namespace: &str, // Todo add query
  ) -> Result<PgGenericCount, NanocldError> {
    let mut res = self.get(String::from("/networks/count")).send().await?;
    let status = res.status();
    is_api_error(&mut res, &status).await?;
    let count = res.json::<PgGenericCount>().await?;
    Ok(count)
  }
}
