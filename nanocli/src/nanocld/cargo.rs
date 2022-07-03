use clap::Parser;
use tabled::Tabled;
use thiserror::Error;
use serde::{Serialize, Deserialize};

use super::{
  client::Nanocld,
  error::{NanocldError, is_api_error},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CargoProxyConfigPartial {
  pub(crate) domain_name: String,
  pub(crate) host_ip: String,
  pub(crate) target_port: i32,
}

#[derive(Debug, Error)]
pub enum CargoProxyConfigError {
  #[error("the config key `{0}` is not available")]
  ParseError(String),
  #[error("the config key `{0}` is required")]
  ValueRequired(String),
}

impl std::str::FromStr for CargoProxyConfigPartial {
  type Err = CargoProxyConfigError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut options = s.split(',');

    let item = options.try_fold(
      CargoProxyConfigPartial {
        domain_name: String::from(""),
        host_ip: String::from(""),
        target_port: 0,
      },
      |acc, option| {
        let args = option.split('=').collect::<Vec<&str>>();
        match args[0] {
          "domain_name" => Ok(CargoProxyConfigPartial {
            domain_name: String::from(args[1]),
            ..acc
          }),
          "host_ip" => Ok(CargoProxyConfigPartial {
            host_ip: String::from(args[1]),
            ..acc
          }),
          "target_port" => Ok(CargoProxyConfigPartial {
            target_port: args[1].parse::<i32>().map_err(|_| {
              CargoProxyConfigError::ParseError(format!(
                "{} must be a number",
                args[0]
              ))
            })?,
            ..acc
          }),
          &_ => Err(CargoProxyConfigError::ParseError(args[0].to_owned())),
        }
      },
    )?;

    if item.target_port == 0 {
      return Err(CargoProxyConfigError::ValueRequired(String::from(
        "target_port",
      )));
    }

    Ok(item)
  }
}

impl std::fmt::Display for CargoProxyConfigPartial {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

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
  /// proxy config is an optional string as follow domain_name=your_domain,host_ip=your_host_ip
  #[clap(long)]
  pub(crate) proxy_config: CargoProxyConfigPartial,
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
  #[tabled(display_with = "optional_string")]
  pub(crate) network: Option<String>,
  #[serde(rename = "namespace_name")]
  pub(crate) namespace: String,
}

fn optional_string(s: &Option<String>) -> String {
  match s {
    None => String::from(""),
    Some(s) => s.to_owned(),
  }
}

impl Nanocld {
  pub async fn list_cargo(&self) -> Result<Vec<CargoItem>, NanocldError> {
    let mut res = self.get(String::from("/cargoes")).send().await?;
    let status = res.status();
    is_api_error(&mut res, &status).await?;
    let items = res.json::<Vec<CargoItem>>().await?;

    Ok(items)
  }

  pub async fn create_cargo(
    &self,
    item: &CargoPartial,
  ) -> Result<CargoItem, NanocldError> {
    let mut res = self.post(String::from("/cargoes")).send_json(item).await?;
    let status = res.status();
    is_api_error(&mut res, &status).await?;
    let item = res.json::<CargoItem>().await?;

    Ok(item)
  }

  pub async fn delete_cargo(
    &self,
    cargo_name: String,
  ) -> Result<(), NanocldError> {
    let mut res = self
      .delete(format!("/cargoes/{name}", name = cargo_name))
      .send()
      .await?;
    let status = res.status();
    is_api_error(&mut res, &status).await?;

    Ok(())
  }
}
