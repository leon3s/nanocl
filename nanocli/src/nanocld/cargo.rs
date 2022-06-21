use clap::Parser;
use tabled::Tabled;
use serde::{Serialize, Deserialize};

use super::{
  client::Nanocld,
  error::{Error, is_api_error},
};

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct CargoPartial {
  pub(crate) name: String,
  /// name of the network to connect to
  #[clap(long, name = "network")]
  pub(crate) network_name: Option<String>,
  /// name of the image
  #[clap(long = "image")]
  pub(crate) image_name: String,
  /// list of open to open
  #[clap(short, long)]
  pub(crate) ports: Option<Vec<String>>,
}

/// Cargo item is an definition to container create image and start them
/// this structure ensure read and write in database
#[derive(Debug, Tabled, Serialize, Deserialize)]
pub struct CargoItem {
  pub(crate) key: String,
  pub(crate) name: String,
  #[serde(rename = "image_name")]
  pub(crate) image: String,
  #[serde(rename = "network_name")]
  pub(crate) network: String,
  #[serde(rename = "namespace_name")]
  pub(crate) namespace: String,
}

impl Nanocld {
  pub async fn list_cargo(&self) -> Result<Vec<CargoItem>, Error> {
    let mut res = self
      .get(String::from("/cargos"))
      .send()
      .await
      .map_err(Error::SendRequest)?;
    let status = res.status();
    is_api_error(&mut res, &status).await?;
    let items = res
      .json::<Vec<CargoItem>>()
      .await
      .map_err(Error::JsonPayload)?;
    Ok(items)
  }

  pub async fn create_cargo(
    &self,
    item: &CargoPartial,
  ) -> Result<CargoItem, Error> {
    let mut res = self
      .post(String::from("/cargos"))
      .send_json(item)
      .await
      .map_err(Error::SendRequest)?;
    let status = res.status();
    is_api_error(&mut res, &status).await?;
    let item = res.json::<CargoItem>().await.map_err(Error::JsonPayload)?;
    Ok(item)
  }

  pub async fn delete_cargo(&self, cargo_name: String) -> Result<(), Error> {
    let mut res = self
      .delete(format!("/cargos/{name}", name = cargo_name))
      .send()
      .await
      .map_err(Error::SendRequest)?;
    let status = res.status();
    is_api_error(&mut res, &status).await?;
    Ok(())
  }

  pub async fn start_cargo(&self, cargo_name: String) -> Result<(), Error> {
    let mut res = self
      .post(format!("/cargos/{name}/start", name = cargo_name))
      .send()
      .await
      .map_err(Error::SendRequest)?;
    let status = res.status();
    is_api_error(&mut res, &status).await?;
    Ok(())
  }
}
