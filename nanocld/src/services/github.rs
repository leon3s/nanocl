use url::{Url, ParseError};
use ntex::http::client::Client;
use serde::{Serialize, Deserialize};

use crate::models::GitRepositoryCreate;

#[derive(Debug, Serialize, Deserialize)]
pub struct GitRepositoryBranch {
  pub(crate) name: String,
}

#[derive(Debug)]
pub struct GitDesc {
  pub(crate) host: String,
  pub(crate) path: String,
}

pub fn parse_git_url(url: &String) -> Result<GitDesc, ParseError> {
  let url_parsed = Url::parse(url)?;

  let host = match url_parsed.host_str() {
    Some(host) => host,
    None => {
      return Err(ParseError::EmptyHost)
    },
  };

  let path = url_parsed.path();

  let result = GitDesc {
    host: host.to_string(),
    path: path.to_string(),
  };

  Ok(result)
}

pub async fn list_branches(
  item: &GitRepositoryCreate,
) -> Result<Vec<GitRepositoryBranch>, Box<dyn std::error::Error + 'static>> {
  let client = Client::new();

  let git_desc = parse_git_url(&item.url)?;

  let url = "https://api.".to_owned() + &git_desc.host + "/repos" + &git_desc.path + "/branches";

  let mut res = client
  .get(url)
  .set_header("Accept", "application/vnd.github.v3+json")
  .set_header("User-Agent", "ntex-client")
  .send()
  .await?;
  let body = res.json::<Vec<GitRepositoryBranch>>().await?;
  Ok(body)
}

#[cfg(test)]
mod test_github {

  use crate::models::{
    GitRepositoryCreate,
    GitRepositorySourceType,
  };

  use super::*;

  use crate::utils::test::*;

  #[ntex::test]
  async fn list_repository_branches() -> TestReturn {
    let item = GitRepositoryCreate {
      name: String::from("express-test-deploy"),
      token: None,
      url: String::from("https://github.com/leon3s/express-test-deploy"),
    };
    let branches = list_branches(&item).await?;
    println!("branches : {:?}", branches);
    Ok(())
  }
}
