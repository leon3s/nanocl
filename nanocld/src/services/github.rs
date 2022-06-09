use serde_json::Value;
use ntex::http::client::{Client, error::SendRequestError};

use crate::models::GitRepositoryCreate;

pub async fn validate_repository(item: &GitRepositoryCreate) -> Result<(), SendRequestError> {

  let client = Client::new();

  let url = "https://api.github.com/repos/".to_owned() + &item.name + "/branches";

  println!("url : {:?}", url);

  let mut res = client
  .get(url)
  .set_header("Accept", "application/vnd.github.v3+json")
  .set_header("User-Agent", "ntex-client")
  .send()
  .await?;

  let body = res.json::<Value>().await;

  println!("github response {:?} body {:?}", res, body);
  Ok(())
}
