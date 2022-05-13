use std::rc::Rc;
use ntex::{http::client::{Client, Connector, ClientRequest}, rt};

#[derive(Clone)]
pub struct DockerClient(Rc<DockerClientConfig>);

pub(self) struct DockerClientConfig {
  pub(self) client: Client,
}

impl DockerClient {
  pub fn new() -> DockerClient {
    DockerClient(Rc::new(DockerClientConfig {
      client: Client::build()
      .connector(Connector::default()
        .connector(ntex::service::fn_service(|_| async {
            Ok(rt::unix_connect("/var/run/docker.sock").await?)
        }))
        .finish(),
      )
      .finish(),
    }))
  }

  fn client(&self) -> Client {
    self.0.client.to_owned()
  }

  fn forge_url(&self, url: String) -> String {
    "http://localhost".to_owned() + &url
  }

  pub fn get(&self, url: String) -> ClientRequest {
    let url_forged = self.forge_url(url);
    self.client().get(url_forged)
  }

  pub fn post(&self, url: String) -> ClientRequest {
    let url_forged = self.forge_url(url);
    self.client().post(url_forged)
  }

  pub fn put(&self, url: String) -> ClientRequest {
    let url_forged = self.forge_url(url);
    self.client().put(url_forged)
  }

  pub fn patch(&self, url: String) -> ClientRequest {
    let url_forged = self.forge_url(url);
    self.client().patch(url_forged)
  }

  pub fn delete(&self, url: String) -> ClientRequest {
    let url_forged = self.forge_url(url);
    self.client().delete(url_forged)
  }
}
