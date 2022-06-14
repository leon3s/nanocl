use ntex::rt;
use std::path::Path;
use ntex::http::client::{Client, Connector, ClientRequest};
use thiserror::Error;
use ntex::http::client::error::{JsonPayloadError, SendRequestError};

use crate::container::Container;

#[derive(Default)]
pub struct HttpClient {
  base_url: String,
  http_client: Client,
}

#[derive(Error, Debug)]
pub enum DockerClientError {
  #[error("json parsing error")]
  JsonPayloadError(JsonPayloadError),
  #[error("connection error")]
  SendRequestError(SendRequestError),
  #[error("urlencode error")]
  UrlEncodeError(serde_urlencoded::ser::Error),
}

impl HttpClient {
  pub fn new(base_url: Option<String>) -> Self {
    let client = Client::build()
      .connector(
        Connector::default()
          .connector(ntex::service::fn_service(move |_| async {
            Ok(rt::unix_connect("/var/run/docker.sock").await?)
          }))
          .finish(),
      )
      .finish();
    HttpClient {
      http_client: client,
      base_url: base_url.unwrap_or_else(|| String::from("http://localhost")),
    }
  }

  fn gen_path(&self, path: impl AsRef<Path>) -> String {
    let append_path = path.as_ref().to_str().unwrap_or_default();
    self.base_url.to_owned() + append_path
  }

  pub fn post(&self, path: impl AsRef<Path>) -> ClientRequest {
    let gen_path = self.gen_path(path);
    self.http_client.post(gen_path)
  }

  pub fn delete(&self, path: impl AsRef<Path>) -> ClientRequest {
    let gen_path = self.gen_path(path);
    self.http_client.delete(gen_path)
  }

  pub fn get(&self, path: impl AsRef<Path>) -> ClientRequest {
    let gen_path = self.gen_path(path);
    self.http_client.get(gen_path)
  }
}

#[derive(Default)]
pub struct DockerClient {
  pub container: Container,
}

impl DockerClient {
  /// Create DockerClient with default settings
  /// this will try to connect to /var/run/docker.sock
  pub fn new() -> Self {
    let client = HttpClient::new(None);
    let container = Container::new(client);
    DockerClient { container }
  }
}

#[cfg(test)]
mod test_client {
  use super::*;

  #[ntex::test]
  async fn test_new() {
    DockerClient::new();
  }
}
