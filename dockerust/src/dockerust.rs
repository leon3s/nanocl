use std::rc::Rc;
pub mod docker_client;
pub mod container;
use container::Container;
use docker_client::DockerClient;

#[derive(Clone)]
pub struct Dockerust(Rc<DockerustConfig>);

pub(self) struct DockerustConfig {
  pub(self) client: DockerClient,
}

impl Default for Dockerust {
  fn default() -> Self {
      Dockerust(Rc::new(DockerustConfig {
          client: DockerClient::new(),
      }))
  }
}

impl Dockerust {
  pub fn new() -> Dockerust {
    Dockerust::default()
  }

  pub fn container(&self) -> Container {
    Container::new(self.0.client.to_owned())
  }
}
