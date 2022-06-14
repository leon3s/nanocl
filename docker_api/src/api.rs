use thiserror::Error;
use ntex::http::error::PayloadError;
use ntex::http::client::error::{JsonPayloadError, SendRequestError};

use crate::image::Image;
use crate::container::Container;

use super::client::HttpClient;

#[derive(Error, Debug)]
pub enum DockerApiError {
  #[error("payload error")]
  Errorpayload(PayloadError),
  #[error("json parsing error")]
  Errorjsonpayload(JsonPayloadError),
  #[error("connection error")]
  Errorsendrequest(SendRequestError),
  #[error("urlencode error")]
  Errorurlencode(serde_urlencoded::ser::Error),
}

#[derive(Default)]
pub struct DockerApi {
  pub image: Image,
  pub container: Container,
}

impl DockerApi {
  /// Create DockerApi with default settings
  /// this will try to connect to /var/run/docker.sock
  pub fn new() -> Self {
    let client = HttpClient::new(None);
    let container = Container::new(client.to_owned());
    let image = Image::new(client);
    DockerApi { image, container }
  }
}

#[cfg(test)]
mod test_client {
  use super::*;

  #[ntex::test]
  async fn test_new() {
    DockerApi::new();
  }
}
