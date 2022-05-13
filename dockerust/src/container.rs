use serde::{Serialize, Deserialize};
use ntex::http::client::{ClientResponse, error::SendRequestError};

use crate::docker_client::DockerClient;

#[derive(Clone)]
pub struct Container {
  client: DockerClient,
}

/** Allow snake case because it's how the docker api work */
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerCreatePayload {
  pub name: String,
  pub image: String,
}

/** Allow snake case because it's how the docker api work */
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerCreateResp {
  pub id: String,
  pub warnings: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerStatQuery {
  pub stream: bool,
}

impl Container {
  pub fn new(client: DockerClient) -> Container {
    Container {
      client
    }
  }

  pub async fn list(&self) -> Result<ClientResponse, SendRequestError> {
    let url = String::from("/containers/json");
    self.client.get(url).send().await
  }

  pub async fn create(&self, new_container: ContainerCreatePayload) -> Result<ClientResponse, SendRequestError> {
    let url = String::from("/containers/create");
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

  pub async fn stats(&self, id: String) {
    let url = format!("/containers/{id}/stats", id = id);
    let query = ContainerStatQuery {
      stream: true,
    };
    let query = self.client.get(url).query(&query).unwrap().send().await.unwrap();
    query.take_payload();
  }
}
