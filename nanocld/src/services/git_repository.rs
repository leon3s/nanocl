use ntex::web;
use ntex::http::StatusCode;

use crate::repositories;
use crate::errors::HttpResponseError;
use crate::models::{Pool, GitRepositoryItem, GitRepositoryBranchItem};

use super::{docker, github};

pub async fn build(
  item: GitRepositoryItem,
  branch_name: &str,
  docker_api: &web::types::State<bollard::Docker>,
  pool: &web::types::State<Pool>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let github_api = github::GithubApi::new();
  // we find the repository by it's unique name
  let live_branch = github_api
    .inspect_branch(&item, branch_name)
    .await
    .map_err(|err| HttpResponseError {
      msg: format!("{:?}", err),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

  let gen_key = item.name.to_owned() + "-" + &item.default_branch;
  let stored_branch =
    repositories::git_repository_branch::get_by_key(gen_key, pool).await?;
  let image_name = item.name.to_owned() + ":" + &live_branch.name;
  let image_exist = docker_api.inspect_image(&image_name).await;
  let new_branch = GitRepositoryBranchItem {
    last_commit_sha: live_branch.commit.sha,
    ..stored_branch
  };
  // We update stored_branch if it's not the lasted stored commit
  if new_branch.last_commit_sha == stored_branch.last_commit_sha {
    repositories::git_repository_branch::update_item(
      new_branch.to_owned(),
      pool,
    )
    .await?;
  }
  match image_exist {
    // Image not exist so we build it
    Err(_) => {
      log::info!("it's first build");
      let rx_body = docker::build_git_repository(
        image_name.to_owned(),
        item,
        new_branch.to_owned(),
        docker_api.to_owned(),
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
      let rx_body = docker::build_git_repository(
        image_name.to_owned(),
        item,
        new_branch.to_owned(),
        docker_api.to_owned(),
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
