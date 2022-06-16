//! File to handle git repository routes
use ntex::http::StatusCode;
use ntex::web;
use serde::{Deserialize, Serialize};

use crate::models::{GitRepositoryBranchPartial, GitRepositoryPartial, Pool};
use crate::repositories::{git_repository, git_repository_branch};
use crate::services::github;

use super::errors::HttpError;

#[derive(Debug, Serialize, Deserialize)]
struct GitRepositoryQuery {
  namespace: Option<String>,
}

/// List all git repository
#[utoipa::path(
  get,
  path = "/git_repositories",
  responses(
      (status = 200, description = "Array of git_repository", body = [GitRepositoryItem]),
  ),
)]
#[web::get("/git_repositories")]
async fn list(
  pool: web::types::State<Pool>,
) -> Result<web::HttpResponse, HttpError> {
  let items = git_repository::list(&pool).await?;

  Ok(web::HttpResponse::Ok().json(&items))
}

/// Create new git repository
#[utoipa::path(
  post,
  path = "/git_repositories",
  request_body = GitRepositoryPartial,
  responses(
    (status = 201, description = "Fresh created git_repository", body = GitRepositoryItem),
    (status = 400, description = "Generic database error"),
    (status = 404, description = "Namespace name not valid"),
    (status = 422, description = "The provided payload is not valid"),
  ),
)]
#[web::post("/git_repositories")]
async fn create(
  pool: web::types::State<Pool>,
  web::types::Json(payload): web::types::Json<GitRepositoryPartial>,
) -> Result<web::HttpResponse, HttpError> {
  let res = github::list_branches(&payload).await;

  let gitbranches = match res {
    Err(_) => {
      return Err(HttpError {
        status: StatusCode::BAD_REQUEST,
        msg: String::from(
          "unable to list branch for this git repository may token missing ?",
        ),
      })
    }
    Ok(branches) => branches,
  };

  let item = git_repository::create(payload, &pool).await?;

  let branches = gitbranches
    .into_iter()
    .map(|branch| GitRepositoryBranchPartial {
      name: branch.name,
      repository_id: item.id,
    })
    .collect::<Vec<GitRepositoryBranchPartial>>();

  git_repository_branch::create_many(branches, &pool).await?;

  Ok(web::HttpResponse::Created().json(&item))
}

/// Delete git repository by it's id or name
#[utoipa::path(
  delete,
  path = "/git_repositories/{id}",
  params(
    ("id" = String, path, description = "Id or name of git repository"),
  ),
  responses(
    (status = 201, description = "Number of entry deleted", body = PgDeleteGeneric),
    (status = 400, description = "Generic database error"),
    (status = 404, description = "Namespace name not valid"),
  ),
)]
#[web::delete("/git_repositories/{id}")]
async fn delete_by_id_or_name(
  pool: web::types::State<Pool>,
  req_path: web::types::Path<String>,
) -> Result<web::HttpResponse, HttpError> {
  let id = req_path.into_inner();
  println!("git repository id to delete {:?}", id);
  let repository = git_repository::find_by_id_or_name(id, &pool).await?;
  git_repository_branch::delete_by_repository_id(repository.id, &pool).await?;
  let res =
    git_repository::delete_by_id_or_name(repository.id.to_string(), &pool)
      .await?;
  Ok(web::HttpResponse::Ok().json(&res))
}

/// Configure ntex to bind our routes
pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(list);
  config.service(create);
  config.service(delete_by_id_or_name);
}

#[cfg(test)]
mod test_namespace_git_repository {
  use crate::models::{GitRepositoryPartial, GitRepositoryItem};
  use crate::utils::test::*;

  use super::ntex_config;

  // Test to list git repositories
  async fn test_list(srv: &TestServer) -> TestReturn {
    let resp = srv.get("/git_repositories").send().await?;

    assert!(resp.status().is_success());
    Ok(())
  }

  // test to create git repository from opensource github
  // and delete it to clean database
  async fn test_create_and_delete_by_name(srv: &TestServer) -> TestReturn {
    let new_repository = GitRepositoryPartial {
      token: None,
      name: String::from("express-test-deploy"),
      url: String::from("https://github.com/leon3s/express-test-deploy"),
    };
    let res = srv
      .post("/git_repositories")
      .send_json(&new_repository)
      .await?;
    assert!(res.status().is_success());
    let mut res = srv
      .delete("/git_repositories/express-test-deploy")
      .send()
      .await?;
    println!("res {:?}", res);
    let body = res.body().await?;
    println!("body : {:?}", body);
    assert!(res.status().is_success());
    Ok(())
  }

  // Create and delete by id a repository
  async fn test_create_and_delete_by_id(srv: &TestServer) -> TestReturn {
    let new_repository = GitRepositoryPartial {
      token: None,
      name: String::from("test-repo2"),
      url: String::from("https://github.com/leon3s/express-test-deploy"),
    };
    let mut res = srv
      .post("/git_repositories")
      .send_json(&new_repository)
      .await?;
    let item = res.json::<GitRepositoryItem>().await?;
    let mut res = srv
      .delete(format!("/git_repositories/{id}", id = item.id))
      .send()
      .await?;
    println!("res : {:?}", res);
    let body = res.body().await?;
    println!("body : {:?}", body);
    assert!(res.status().is_success());
    Ok(())
  }

  #[ntex::test]
  async fn main() -> TestReturn {
    let srv = generate_server(ntex_config);

    test_list(&srv).await?;
    test_create_and_delete_by_id(&srv).await?;
    test_create_and_delete_by_name(&srv).await?;
    Ok(())
  }
}
