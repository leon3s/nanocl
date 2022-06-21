use clap::Parser;
use tabled::Tabled;
use serde::{Serialize, Deserialize};

use super::client::Nanocld;
use super::error::{Error, is_api_error};

#[derive(Tabled, Serialize, Deserialize)]
pub struct NamespaceItem {
  pub name: String,
}

#[derive(Debug, Parser)]
pub struct NamespacePartial {
  pub name: String,
}

impl Nanocld {
  pub async fn list_namespace(&self) -> Result<Vec<NamespaceItem>, Error> {
    let mut res = self
      .get(String::from("/namespaces"))
      .send()
      .await
      .map_err(Error::SendRequest)?;

    let status = res.status();
    is_api_error(&mut res, &status).await?;
    let items = res
      .json::<Vec<NamespaceItem>>()
      .await
      .map_err(Error::JsonPayload)?;
    Ok(items)
  }

  pub async fn create_namespace(
    &self,
    name: String,
  ) -> Result<NamespaceItem, Error> {
    let new_item = NamespaceItem { name };
    let mut res = self
      .post(String::from("/namespaces"))
      .send_json(&new_item)
      .await
      .map_err(Error::SendRequest)?;
    let status = res.status();
    is_api_error(&mut res, &status).await?;
    let item = res
      .json::<NamespaceItem>()
      .await
      .map_err(Error::JsonPayload)?;
    Ok(item)
  }
}
