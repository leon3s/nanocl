//! File to handle git repository routes
use ntex::web;
use ntex::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{services, repositories};
use crate::models::{Pool, GitRepositoryPartial, GitRepositoryBranchPartial};

use crate::errors::HttpResponseError;

#[derive(Debug, Serialize, Deserialize)]
pub struct GitRepositoryBuildQuery {
  branch: Option<String>,
}

/// List all git repository
#[cfg_attr(feature = "openapi", utoipa::path(
  get,
  path = "/git_repositories",
  responses(
      (status = 200, description = "Array of git_repository", body = [GitRepositoryItem]),
  ),
))]
#[web::get("/git_repositories")]
async fn list_git_repository(
  pool: web::types::State<Pool>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let items = repositories::git_repository::list(&pool).await?;

  Ok(web::HttpResponse::Ok().json(&items))
}

/// Create new git repository
#[cfg_attr(feature = "openapi", utoipa::path(
  post,
  path = "/git_repositories",
  request_body = GitRepositoryPartial,
  responses(
    (status = 201, description = "Fresh created git_repository", body = GitRepositoryItem),
    (status = 400, description = "Generic database error"),
    (status = 404, description = "Namespace name not valid"),
    (status = 422, description = "The provided payload is not valid"),
  ),
))]
#[web::post("/git_repositories")]
async fn create_git_repository(
  pool: web::types::State<Pool>,
  web::types::Json(payload): web::types::Json<GitRepositoryPartial>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let github_api = services::github::GithubApi::new();
  let repo =
    github_api
      .sync_repo(&payload)
      .await
      .map_err(|err| HttpResponseError {
        msg: format!("{:?}", err),
        status: StatusCode::BAD_REQUEST,
      })?;
  let branches = github_api.list_branches(&payload).await.map_err(|err| {
    HttpResponseError {
      msg: format!("{:?}", err),
      status: StatusCode::BAD_REQUEST,
    }
  })?;

  let item =
    repositories::git_repository::create(payload, repo.default_branch, &pool)
      .await?;

  let branches = branches
    .into_iter()
    .map(|branch| GitRepositoryBranchPartial {
      name: branch.name,
      last_commit_sha: branch.commit.sha,
      repository_name: item.name.clone(),
    })
    .collect::<Vec<GitRepositoryBranchPartial>>();

  repositories::git_repository_branch::create_many(branches, &pool).await?;

  Ok(web::HttpResponse::Created().json(&item))
}

/// Delete git repository by it's name
#[cfg_attr(feature = "openapi", utoipa::path(
  delete,
  path = "/git_repositories/{name}",
  params(
    ("id" = String, path, description = "Name of git repository"),
  ),
  responses(
    (status = 201, description = "Number of entry deleted", body = PgDeleteGeneric),
    (status = 400, description = "Generic database error"),
    (status = 404, description = "Namespace name not valid"),
  ),
))]
#[web::delete("/git_repositories/{id}")]
async fn delete_git_repository_by_name(
  pool: web::types::State<Pool>,
  req_path: web::types::Path<String>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let id = req_path.into_inner();
  let repository =
    repositories::git_repository::find_by_name(id, &pool).await?;
  repositories::git_repository_branch::delete_by_repository_id(
    repository.name.to_owned(),
    &pool,
  )
  .await?;
  let res = repositories::git_repository::delete_by_name(
    repository.name.to_string(),
    &pool,
  )
  .await?;
  Ok(web::HttpResponse::Ok().json(&res))
}

#[web::post("/git_repositories/{id}/build")]
async fn build_git_repository(
  pool: web::types::State<Pool>,
  docker_api: web::types::State<bollard::Docker>,
  id: web::types::Path<String>,
  web::types::Query(qs): web::types::Query<GitRepositoryBuildQuery>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let id = id.into_inner();
  let git_repo = repositories::git_repository::find_by_name(id, &pool).await?;

  let branch_name = match qs.branch {
    None => git_repo.default_branch.to_owned(),
    Some(branch) => branch,
  };

  services::git_repository::build(git_repo, &branch_name, &docker_api, &pool)
    .await
}

/// Configure ntex to bind our routes
pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(list_git_repository);
  config.service(build_git_repository);
  config.service(create_git_repository);
  config.service(delete_git_repository_by_name);
}

#[cfg(test)]
mod test_namespace_git_repository {
  use crate::models::GitRepositoryPartial;
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
      name: String::from("express-test-deploy"),
      url: String::from("https://github.com/leon3s/express-test-deploy"),
    };
    let res = srv
      .post("/git_repositories")
      .send_json(&new_repository)
      .await?;
    assert!(res.status().is_success());

    let res = srv
      .delete("/git_repositories/express-test-deploy")
      .send()
      .await?;
    assert!(res.status().is_success());
    Ok(())
  }

  // test to create git repository from opensource github
  // and delete it to clean database
  async fn test_create_and_build_and_delete_by_name(
    srv: &TestServer,
  ) -> TestReturn {
    let new_repository = GitRepositoryPartial {
      name: String::from("express-test"),
      url: String::from("https://github.com/leon3s/express-test-deploy"),
    };
    let res = srv
      .post("/git_repositories")
      .send_json(&new_repository)
      .await?;
    assert!(res.status().is_success());
    let res = srv.delete("/git_repositories/express-test").send().await?;
    assert!(res.status().is_success());
    Ok(())
  }

  #[ntex::test]
  async fn main() -> TestReturn {
    let srv = generate_server(ntex_config).await;

    test_list(&srv).await?;
    test_create_and_delete_by_name(&srv).await?;
    test_create_and_build_and_delete_by_name(&srv).await?;
    Ok(())
  }
}
