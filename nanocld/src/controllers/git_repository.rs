/**
 * HTTP Method to administrate git_repositories
 */
use ntex::web;
use serde::{Serialize, Deserialize};

use crate::services::github;
use crate::repositories::git_repository;
use crate::models::{Pool, GitRepositoryCreate};


use super::errors::HttpError;

#[derive(Debug, Serialize, Deserialize)]
struct GitRepositoryQuery {
  namespace: Option<String>,
}

/// Endpoint to get list of git repositories for a given namespace
#[utoipa::path(
  get,
  path = "/git_repositories",
  responses(
      (status = 200, description = "Array of git_repository", body = GitRepositoryItem),
  ),
)]
#[web::get("/git_repositories")]
async fn list(
    pool: web::types::State<Pool>,
) -> Result<web::HttpResponse, HttpError> {
  let items = git_repository::list(
    &pool,
  ).await?;

  Ok(web::HttpResponse::Ok().json(&items))
}

/// Endpoint to create a git repository in given namespace
#[utoipa::path(
  post,
  path = "/git_repositories",
  request_body = GitRepositoryCreate,
  params(
    ("namespace" = Option<String>, query, description = "Namespace to add git repository in if empty we use 'default' as value"),
  ),
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
    web::types::Json(payload): web::types::Json<GitRepositoryCreate>,
) -> Result<web::HttpResponse, HttpError> {

  let res = github::validate_repository(&payload).await;

  let item = git_repository::create(
    payload,
    &pool,
  ).await?;

  Ok(
    web::HttpResponse::Created()
    .json(&item)
  )
}

/// Endpoint to delete a git repository by it's id or name for given namespace
#[utoipa::path(
  delete,
  path = "/git_repositories/{id}",
  params(
    ("id" = String, path, description = "Id or name of git repository"),
    ("namespace" = Option<String>, query, description = "Namespace to add git repository in if empty we use 'default' as value"),
  ),
  responses(
    (status = 201, description = "Number of entry deleted", body = PgDeleteGeneric),
    (status = 400, description = "Generic database error"),
    (status = 404, description = "Namespace name not valid"),
  ),
)]
#[web::delete("/git_repositories/{id}*")]
async fn delete_by_id_or_name(
  pool: web::types::State<Pool>,
  req_path: web::types::Path<String>,
) -> Result<web::HttpResponse, HttpError> {
  let id = req_path.into_inner();
  println!("git repository id to delete {:?}", id);
  let res = git_repository::delete_by_id_or_name(id, &pool).await?;
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
  use crate::utils::test::*;
  use crate::models::{
    GitRepositoryItem,
    GitRepositoryCreate,
    GitRepositorySourceType,
  };

  use super::ntex_config;  

  /// Test list route
  async fn test_list(srv: &TestServer) -> TestReturn {
    let resp = srv
    .get("/git_repositories")
    .send().await?;

    assert!(resp.status().is_success());
    Ok(())
  }

  async fn test_create(srv: &TestServer) -> TestReturn {
    let new_repository = GitRepositoryCreate {
        name: String::from("leon3s/express-test-deploy"),
        token: None,
        source: GitRepositorySourceType::Github,
    };
    let res = srv
    .post("/git_repositories")
    .send_json(&new_repository)
    .await?;
    assert!(res.status().is_success());
    Ok(())
  }

  async fn test_delete_by_id(srv: &TestServer) -> TestReturn {
    let new_repository = GitRepositoryCreate {
      name: String::from("test-user/test-repo2"),
      token: None,
      source: GitRepositorySourceType::Github,
    };
    let mut res = srv
    .post("/git_repositories")
    .send_json(&new_repository)
    .await?;

    let item = res.json::<GitRepositoryItem>().await?;

    let mut res = srv.delete(format!("/git_repositories/{id}", id = item.id)).send().await?;

    println!("res : {:?}", res);

    let body = res.body().await?;

    println!("body : {:?}", body);

    // assert!(res.status().is_success());
    Ok(())
  }

  async fn test_create_fail(srv: &TestServer) -> TestReturn {
    Ok(())
  }

  async fn test_delete_by_name(srv: &TestServer) -> TestReturn {
    let mut res = srv
    .delete("/git_repositories/leon3s/express-test-deploy")
    .send().await?;
    println!("res {:?}", res);
    let body = res.body().await?;
    println!("body : {:?}", body);
    // assert!(res.status().is_success());
    Ok(())
  }

  #[ntex::test]
  async fn main() -> TestReturn {
    let srv = generate_server(ntex_config);

    test_list(&srv).await?;
    test_create(&srv).await?;
    test_delete_by_name(&srv).await?;
    test_delete_by_id(&srv).await?;
    Ok(())
  }
}
