use thiserror::Error;
use ntex::http::client::error::{JsonPayloadError, SendRequestError};

use crate::client::HttpClient;
use crate::container::Container;

#[derive(Error, Debug)]
pub enum DockerApiError {
  #[error("json parsing error")]
  Errorjsonpayload(JsonPayloadError),
  #[error("connection error")]
  Errorsendrequest(SendRequestError),
  #[error("urlencode error")]
  Errorurlencode(serde_urlencoded::ser::Error),
}

#[derive(Default)]
pub struct DockerApi {
  pub container: Container,
}

impl DockerApi {
  /// Create DockerApi with default settings
  /// this will try to connect to /var/run/docker.sock
  pub fn new() -> Self {
    let client = HttpClient::new(None);
    let container = Container::new(client);
    DockerApi { container }
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
