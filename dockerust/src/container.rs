use serde::{Serialize, Deserialize};
use ntex::{http::client::{ClientResponse, error::SendRequestError}, util::HashMap};

use crate::docker_client::DockerClient;

#[derive(Clone)]
pub struct Container {
  client: DockerClient,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerCreate {
  name: String,
  image_name: String,
  ports: Option<HashMap<String, u64>>,
}

impl Container {
  pub fn new(client: DockerClient) -> Container {
    Container {
      client
    }
  }

  pub async fn list(&self) -> Result<ClientResponse, SendRequestError> {
    let url = "/containers/json".to_string(); 
    self.client.get(url).send().await
  }

  pub async fn create(&self, new_container: ContainerCreate) -> Result<ClientResponse, SendRequestError> {
    let url = "/containers".to_string();
    self.client.post(url).send_json(&new_container).await
  }

  pub async fn delete(&self, id: String) -> Result<ClientResponse, SendRequestError> {
    let url = format!("/containers/{id}", id = id);
    self.client.delete(url).send().await
  }

  pub async fn inspect(&self, id: String) -> Result<ClientResponse, SendRequestError> {
    let url = format!("/containers/inspect/{id}", id = id);
    self.client.get(url).send().await
  }
}
