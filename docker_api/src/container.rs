use serde::Serialize;

use crate::client::{HttpClient, DockerClientError};

use crate::models::{
  ContainerSummary, ContainerConfig, ContainerCreateResponse,
  CreateContainerOptions, RemoveContainerOptions, StartContainerOptions,
  StopContainerOptions,
};

#[derive(Default)]
pub struct Container {
  pub(crate) client: HttpClient,
}

impl Container {
  pub(crate) fn new(client: HttpClient) -> Self {
    Container { client }
  }

  /// ---
  ///
  /// # List Containers
  ///
  /// Returns a list of containers.
  ///
  /// # Arguments
  ///
  ///  - Optional [ListContainersOptions](ListContainersOptions) struct.
  ///
  /// # Returns
  ///
  ///  - Vector of [ContainerSummary](ContainerSummary), wrapped in a Future.
  ///
  /// # Examples
  ///
  /// ```rust
  /// # use docker_api::DockerClient;
  /// # let docker = DockerClient::new();
  /// use docker_api::models::ListContainersOptions;
  ///
  /// use std::collections::HashMap;
  /// use std::default::Default;
  ///
  /// let mut filters = HashMap::new();
  /// filters.insert("health", vec!["unhealthy"]);
  ///
  /// let options = Some(ListContainersOptions {
  ///     all: true,
  ///     filters,
  ///     ..Default::default()
  /// });
  ///
  /// docker.container.list(options);
  /// ```
  pub async fn list(&self) -> Result<Vec<ContainerSummary>, DockerClientError> {
    let mut res = match self.client.get("/containers/json").send().await {
      Err(err) => return Err(DockerClientError::SendRequestError(err)),
      Ok(res) => res,
    };
    println!("res : {:?}", res);
    match res.json::<Vec<ContainerSummary>>().await {
      Err(err) => Err(DockerClientError::JsonPayloadError(err)),
      Ok(containers) => Ok(containers),
    }
  }

  /// ---
  ///
  /// # Create Container
  ///
  /// Prepares a container for a subsequent start operation.
  ///
  /// # Arguments
  ///
  ///  - Optional [Create Container Options](CreateContainerOptions) struct.
  ///  - Container [Config](Config) struct.
  ///
  /// # Returns
  ///
  ///  - [ContainerCreateResponse](ContainerCreateResponse), wrapped in a Future.
  ///
  /// # Examples
  ///
  /// ```rust
  /// # use docker_api::DockerClient;
  /// # use docker_api::models::{CreateContainerOptions, ContainerConfig};
  /// # let docker = DockerClient::new();
  ///
  /// use std::default::Default;
  ///
  /// let options = Some(CreateContainerOptions {
  ///     name: String::from("test-container"),
  /// });
  ///
  /// let config = ContainerConfig {
  ///     image: Some(String::from("nginx")),
  ///     ..Default::default()
  /// };
  /// docker.container.create(options, config);
  /// ```
  pub async fn create<T>(
    &self,
    options: Option<CreateContainerOptions<T>>,
    config: ContainerConfig,
  ) -> Result<ContainerCreateResponse, DockerClientError>
  where
    T: Into<String> + Serialize,
  {
    let req = match self.client.post("/containers/create").query(&options) {
      Err(err) => return Err(DockerClientError::UrlEncodeError(err)),
      Ok(req) => req.send_json(&config).await,
    };
    let mut res = match req {
      Err(err) => return Err(DockerClientError::SendRequestError(err)),
      Ok(res) => res,
    };
    println!("create res : {:?}", res);
    match res.json::<ContainerCreateResponse>().await {
      Err(err) => Err(DockerClientError::JsonPayloadError(err)),
      Ok(body) => Ok(body),
    }
  }

  /// ---
  ///
  /// # Start Container
  ///
  /// Starts a container, after preparing it with the [Create Container
  /// API](struct.Docker.html#method.create_container).
  ///
  /// # Arguments
  ///
  ///  - Container id or name as a string slice.
  ///  - Optional [Start Container Options](StartContainerOptions) struct.
  ///
  /// # Returns
  ///
  ///  - unit type `()`, wrapped in a Future.
  ///
  /// # Examples
  ///
  /// ```rust
  /// # use docker_api::DockerClient;
  /// # let docker = DockerClient::new();
  /// use docker_api::models::StartContainerOptions;
  ///
  /// docker.container.start("hello-world", None::<StartContainerOptions<String>>);
  /// ```
  pub async fn start<T>(
    &self,
    id: &str,
    options: Option<StartContainerOptions<T>>,
  ) -> Result<(), DockerClientError>
  where
    T: Into<String> + Serialize,
  {
    let req = match self
      .client
      .post(format!("/containers/{id}/start", id = id))
      .query(&options)
    {
      Err(err) => return Err(DockerClientError::UrlEncodeError(err)),
      Ok(req) => req.send().await,
    };
    println!("start req : {:?}", req);
    match req {
      Err(err) => Err(DockerClientError::SendRequestError(err)),
      Ok(_) => Ok(()),
    }
  }

  /// ---
  ///
  /// # Remove Container
  ///
  /// Remove a container.
  ///
  /// # Arguments
  ///
  /// - Container id or name as a string slice.
  /// - Optional [Remove Container Options](RemoveContainerOptions) struct.
  ///
  /// # Returns
  ///
  ///  - unit type `()`, wrapped in a Future.
  ///
  /// # Examples
  ///
  /// ```rust
  /// # use docker_api::DockerClient;
  /// # let docker = DockerClient::new();
  ///
  /// use docker_api::models::RemoveContainerOptions;
  ///
  /// use std::default::Default;
  ///
  /// let options = Some(RemoveContainerOptions {
  ///     force: true,
  ///     ..Default::default()
  /// });
  ///
  /// docker.container.remove("hello-world", options);
  /// ```
  pub async fn remove(
    &self,
    id: &str,
    options: Option<RemoveContainerOptions>,
  ) -> Result<(), DockerClientError> {
    let req = match self
      .client
      .delete(format!("/containers/{id}", id = id))
      .query(&options)
    {
      Err(err) => return Err(DockerClientError::UrlEncodeError(err)),
      Ok(req) => req.send().await,
    };

    match req {
      Err(err) => Err(DockerClientError::SendRequestError(err)),
      Ok(_) => Ok(()),
    }
  }

  /// ---
  ///
  /// # Stop Container
  ///
  /// Stops a container.
  ///
  /// # Arguments
  ///
  /// - Container name as string slice.
  /// - Optional [Stop Container Options](StopContainerOptions) struct.
  ///
  /// # Returns
  ///
  ///  - unit type `()`, wrapped in a Future.
  ///
  /// # Examples
  ///
  /// ```rust
  /// # use docker_api::DockerClient;
  /// use docker_api::models::StopContainerOptions;
  /// # let docker = DockerClient::new();
  ///
  /// let options = Some(StopContainerOptions{
  ///     t: 30,
  /// });
  ///
  /// docker.container.stop("hello-world", options);
  /// ```
  pub async fn stop(
    &self,
    id: &str,
    options: Option<StopContainerOptions>,
  ) -> Result<(), DockerClientError> {
    let req = match self
      .client
      .post(format!("/containers/{id}/stop", id = id))
      .query(&options)
    {
      Err(err) => return Err(DockerClientError::UrlEncodeError(err)),
      Ok(req) => req.send().await,
    };
    match req {
      Err(err) => Err(DockerClientError::SendRequestError(err)),
      Ok(_) => Ok(()),
    }
  }
}

#[cfg(test)]
mod test_container {
  use super::*;

  #[ntex::test]
  async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    const CONTAINER_NAME: &str = "my-new-container";
    let client = HttpClient::new(None);
    let container = Container::new(client);

    // test list
    let res = container.list().await;
    println!("{:?}", res);
    res?;

    // test create
    let options = Some(CreateContainerOptions {
      name: String::from(CONTAINER_NAME),
    });
    let config = ContainerConfig {
      image: Some(String::from("nginx")),
      ..Default::default()
    };
    let res = container.create(options, config).await;
    println!("{:?}", res);
    res?;

    // test start
    container
      .start(CONTAINER_NAME, None::<StartContainerOptions<String>>)
      .await?;

    // test stop
    container.stop(CONTAINER_NAME, None).await?;

    // test remove
    let options = Some(RemoveContainerOptions {
      force: false,
      ..Default::default()
    });

    container.remove(CONTAINER_NAME, options).await?;
    Ok(())
  }
}
