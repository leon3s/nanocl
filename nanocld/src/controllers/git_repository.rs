//! File to handle git repository routes
use ntex::web;
use ntex::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::services::docker::build_git_repository;
use crate::repositories::{git_repository, git_repository_branch};
use crate::models::{
  Pool, GitRepositoryPartial, GitRepositoryBranchPartial,
  GitRepositoryBranchItem,
};
use crate::services::github::GithubApi;

use crate::errors::HttpResponseError;

#[derive(Debug, Serialize, Deserialize)]
struct GitRepositoryQuery {
  namespace: Option<String>,
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
  let items = git_repository::list(&pool).await?;

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
  let github_api = GithubApi::new();
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
    git_repository::create(payload, repo.default_branch, &pool).await?;

  let branches = branches
    .into_iter()
    .map(|branch| GitRepositoryBranchPartial {
      name: branch.name,
      last_commit_sha: branch.commit.sha,
      repository_name: item.name.clone(),
    })
    .collect::<Vec<GitRepositoryBranchPartial>>();

  git_repository_branch::create_many(branches, &pool).await?;

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
  let repository = git_repository::find_by_name(id, &pool).await?;
  git_repository_branch::delete_by_repository_id(
    repository.name.to_owned(),
    &pool,
  )
  .await?;
  let res =
    git_repository::delete_by_name(repository.name.to_string(), &pool).await?;
  Ok(web::HttpResponse::Ok().json(&res))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitRepositoryBuildQuery {
  branch: Option<String>,
}

/// Build docker image for given git repository name
#[cfg_attr(feature = "openapi", utoipa::path(
  post,
  path = "/git_repositories/{name}/build",
  params(
    ("name" = String, path, description = "Name of git repository"),
  ),
  responses(
    (status = 200, description = "Stream of building process", body = String, content_type = "nanocl/streaming-v1"),
    (status = 304, description = "Content is up to date with selected branch"),
    (status = 400, description = "Generic database error"),
    (status = 404, description = "Namespace name or git repository name not valid"),
  ),
))]
#[web::post("/git_repositories/{name}/build")]
async fn build_git_repository_by_name(
  pool: web::types::State<Pool>,
  docker_api: web::types::State<bollard::Docker>,
  name: web::types::Path<String>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let name = name.into_inner();
  let github_api = GithubApi::new();
  log::info!("requesting build git repository {}", name.to_owned());
  // we find the repository by it's unique name
  let item = git_repository::find_by_name(name.to_owned(), &pool).await?;
  let live_branch = github_api
    .inspect_branch(&item, item.default_branch.to_owned())
    .await
    .map_err(|err| HttpResponseError {
      msg: format!("{:?}", err),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

  let gen_key = item.name.to_owned() + "-" + &item.default_branch;
  let stored_branch = git_repository_branch::get_by_key(gen_key, &pool).await?;
  let image_name = item.name.to_owned() + ":" + &live_branch.name;
  let image_exist = docker_api.inspect_image(&image_name).await;
  let new_branch = GitRepositoryBranchItem {
    last_commit_sha: live_branch.commit.sha,
    ..stored_branch
  };
  // We update stored_branch if it's not the lasted stored commit
  if new_branch.last_commit_sha == stored_branch.last_commit_sha {
    git_repository_branch::update_item(new_branch.to_owned(), &pool).await?;
  }
  match image_exist {
    // Image not exist so we build it
    Err(_) => {
      log::info!("it's first build");
      let rx_body = build_git_repository(
        image_name.to_owned(),
        item,
        new_branch.to_owned(),
        docker_api,
      )
      .await?;
      Ok(
        web::HttpResponse::Ok()
          .content_type("nanocl/streaming-v1")
          .streaming(rx_body),
      )
    }
    Ok(res) => {
      log::info!("we found an image");
      let image_id = res
        .id
        .ok_or_else(|| HttpResponseError {
          msg: String::from("Image is found but we cannot read his id"),
          status: StatusCode::INTERNAL_SERVER_ERROR,
        })?
        .replace("sha256:", "");
      let config = res.config.ok_or_else(|| HttpResponseError {
        msg: String::from("Image is found but we cannot read his config"),
        status: StatusCode::INTERNAL_SERVER_ERROR,
      })?;
      let labels = config.labels.ok_or_else(|| HttpResponseError {
        msg: String::from("Image is found but we cannot read his labels"),
        status: StatusCode::INTERNAL_SERVER_ERROR,
      })?;
      let commit = labels.get("commit").ok_or_else(|| HttpResponseError {
        msg: String::from("Image is found but we cannot get his commit"),
        status: StatusCode::INTERNAL_SERVER_ERROR,
      })?;
      // if image have the latest commit we are up to date.
      // ps i love pointers
      if *commit == new_branch.last_commit_sha {
        log::info!("seems we are up to date!");
        return Ok(web::HttpResponse::NotModified().into());
      }
      let backup_image_name = image_name.to_owned() + "-backup";
      let backup_image_exist =
        docker_api.inspect_image(&backup_image_name).await;
      match backup_image_exist {
        // No backup image so we tag current one has backup
        Err(_) => {
          log::info!("tagging existing image has backup {}", &image_id);
          let tag_options = Some(bollard::image::TagImageOptions {
            tag: new_branch.name.to_owned() + "-backup",
            repo: item.name.to_owned(),
          });
          docker_api.tag_image(&image_id, tag_options).await.map_err(
            |err| HttpResponseError {
              msg: format!("tag error {:?}", err),
              status: StatusCode::INTERNAL_SERVER_ERROR,
            },
          )?;
        }
        Ok(_) => {
          // if it exist we delete the older one
          log::info!("a backup exist deleting it");
          docker_api
            .remove_image(&backup_image_name, None, None)
            .await
            .map_err(|err| HttpResponseError {
              msg: format!("unable to remove image {:?}", err),
              status: StatusCode::INTERNAL_SERVER_ERROR,
            })?;
          log::info!("tagging existing image has backup");
          let tag_options = Some(bollard::image::TagImageOptions {
            tag: new_branch.name.to_owned() + "-backup",
            repo: item.name.to_owned(),
          });
          docker_api.tag_image(&image_id, tag_options).await.map_err(
            |err| HttpResponseError {
              msg: format!("Unable to tag image {:?}", err),
              status: StatusCode::INTERNAL_SERVER_ERROR,
            },
          )?;
        }
      }
      // unless we build the image :O
      let rx_body = build_git_repository(
        image_name.to_owned(),
        item,
        new_branch.to_owned(),
        docker_api,
      )
      .await?;

      Ok(
        web::HttpResponse::Ok()
          .content_type("nanocl/streaming-v1")
          .streaming(rx_body),
      )
    }
  }
}

/// Configure ntex to bind our routes
pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(list_git_repository);
  config.service(create_git_repository);
  config.service(build_git_repository_by_name);
  config.service(delete_git_repository_by_name);
}

#[cfg(test)]
mod test_namespace_git_repository {
  use crate::utils::test::*;
  use crate::models::{GitRepositoryPartial, GitRepositoryItem};

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

  // Create and delete by id a repository
  async fn test_create_and_delete_by_id(srv: &TestServer) -> TestReturn {
    let new_repository = GitRepositoryPartial {
      name: String::from("test-repo2"),
      url: String::from("https://github.com/leon3s/express-test-deploy"),
    };
    let mut res = srv
      .post("/git_repositories")
      .send_json(&new_repository)
      .await?;
    let item = res.json::<GitRepositoryItem>().await?;
    let res = srv
      .delete(format!("/git_repositories/{id}", id = item.name))
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
    test_create_and_delete_by_id(&srv).await?;
    test_create_and_delete_by_name(&srv).await?;
    test_create_and_build_and_delete_by_name(&srv).await?;
    Ok(())
  }
}
