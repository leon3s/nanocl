use tabled::Tabled;
use clap::{Parser, arg_enum};
use serde::{Serialize, Deserialize};
use futures::{TryStreamExt, StreamExt};

use super::client::Nanocld;

use super::error::Error;

arg_enum! {
  #[derive(Debug, Tabled, Serialize, Deserialize)]
  #[serde(rename_all = "lowercase")]
  pub enum GitRepositorySourceType {
    Github,
    Gitlab,
    Local,
  }
}

#[derive(Serialize, Deserialize)]
pub struct GithubRepositoryBuildStream {
  pub stream: Option<String>,
}

#[derive(Tabled, Serialize, Deserialize)]
pub struct GitRepositoryItem {
  pub(crate) name: String,
  pub(crate) url: String,
  pub(crate) default_branch: String,
  pub(crate) source: GitRepositorySourceType,
}

#[derive(Debug, Parser, Serialize)]
pub struct GitRepositoryPartial {
  pub(crate) name: String,
  #[clap(long)]
  pub(crate) url: String,
}

impl Nanocld {
  pub async fn list_git_repository(
    &self,
  ) -> Result<Vec<GitRepositoryItem>, Error> {
    let mut res = self
      .get(String::from("/git_repositories"))
      .send()
      .await
      .map_err(Error::SendRequest)?;

    let items = res
      .json::<Vec<GitRepositoryItem>>()
      .await
      .map_err(Error::JsonPayload)?;

    Ok(items)
  }

  pub async fn create_git_repository(
    &self,
    item: &GitRepositoryPartial,
  ) -> Result<GitRepositoryItem, Error> {
    let mut res = self
      .post(String::from("/git_repositories"))
      .send_json(&item)
      .await
      .map_err(Error::SendRequest)?;

    let body = res
      .json::<GitRepositoryItem>()
      .await
      .map_err(Error::JsonPayload)?;
    Ok(body)
  }

  pub async fn build_git_repository(&self, name: String) -> Result<(), Error> {
    let res = self
      .post(format!("/git_repositories/{name}/build", name = name))
      .send()
      .await
      .map_err(Error::SendRequest)?;

    let mut stream = res.into_stream();
    while let Some(result) = stream.next().await {
      let result = result.map_err(Error::Payload)?;
      let result = &String::from_utf8(result.to_vec()).unwrap();
      let json =
        serde_json::from_str::<GithubRepositoryBuildStream>(result).unwrap();
      print!("{}", json.stream.unwrap_or_default());
    }
    Ok(())
  }

  pub async fn delete_git_repository(&self, name: String) -> Result<(), Error> {
    self
      .delete(format!("/git_repositories/{name}", name = name))
      .send()
      .await
      .map_err(Error::SendRequest)?;
    Ok(())
  }
}
