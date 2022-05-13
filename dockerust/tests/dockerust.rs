#[cfg(test)]
mod dockerust_tests {
  use dockerust::Dockerust;
  use ntex::http::client::error::SendRequestError;

  #[ntex::test]
  async fn get_containers() -> Result<(), SendRequestError> {
    let dockerust = Dockerust::new();
    let resp = dockerust.container().list().await?;
    assert!(resp.status().is_success());
    Ok(())
  }
}
