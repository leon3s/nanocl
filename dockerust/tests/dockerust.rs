#[cfg(test)]
mod dockerust_tests {
  use dockerust::Dockerust;
  use dockerust::container::{ContainerCreatePayload, ContainerCreateResp};
  use ntex::http::client::error::SendRequestError;

  /** Configuration for tests */
  static CONTAINER_IMAGE: &str = "nginx";

  async fn container_create(dockerust: &Dockerust) -> Result<ContainerCreateResp, SendRequestError> {
    let container_payload = ContainerCreatePayload {
      name: String::from("test_container"),
      image: String::from(CONTAINER_IMAGE),
    };
    let mut resp = dockerust.container().create(container_payload).await?;
    let body = resp.json::<ContainerCreateResp>().await.unwrap();
    assert!(resp.status().is_success());
    Ok(body)
  }

  async fn container_delete(dockerust: &Dockerust, id: String) -> Result<(), SendRequestError> {
    let resp = dockerust.container().delete(id).await?;
    assert!(resp.status().is_success());
    Ok(())
  }

  #[ntex::test]
  /** Test dockerust container api */
  async fn container() -> Result<(), SendRequestError> {
    let dockerust = Dockerust::new();
    // Create container with CONTAINER_IMAGE as example
    let container = container_create(&dockerust).await?;
    println!("container: {:?}", container);
    // delete container
    container_delete(&dockerust, container.id).await?;
    Ok(())
  }
}
