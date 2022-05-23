/**
 * HTTP Method to administrate git_repositories
 */
use ntex::web;

use crate::repositories::git_repository;
use crate::models::{
  Pool,
  GitRepositoryCreate,
};

use super::utils::get_poll_conn;
use super::http_error::{
  HttpError,
  db_bloking_error,
};

#[utoipa::path(
  get,
  path = "/git_repositories",
  responses(
      (status = 200, description = "Array of git_repository", body = GitRepositoryItem),
  ),
)]
#[web::get("/git_repositories")]
async fn list(
  poll: web::types::State<Pool>,
) -> Result<web::HttpResponse, HttpError>{
  let conn = get_poll_conn(poll)?;
  let res = web::block(move ||
    git_repository::find_all(&conn)
  ).await;

  match res {
    Err(err) => {
      Err(db_bloking_error(err))
    },
    Ok(items) => {
      Ok(
        web::HttpResponse::Ok()
        .json(&items)
      )
    }
  }
}

#[utoipa::path(
  post,
  path = "/git_repositories",
  request_body = GitRepositoryCreate,
  responses(
    (status = 201, description = "Fresh created git_repository", body = GitRepositoryItem),
    (status = 400, description = "Generic database error"),
    (status = 422, description = "The provided payload is not valid"),
  ),
)]
#[web::post("/git_repositories")]
async fn create(
  pool: web::types::State<Pool>,
  payload: web::types::Json<GitRepositoryCreate>,
) -> Result<web::HttpResponse, HttpError>{
  let jsonp = payload.into_inner();
  let conn = get_poll_conn(pool)?;

  let res = web::block(move ||
    git_repository::create(jsonp, &conn)
  ).await;

  match res {
    Err(err) => {
      eprintln!("db error : {}", err);
      Err(db_bloking_error(err))
    },
    Ok(git_repository) => {
      Ok(
        web::HttpResponse::Created()
        .json(&git_repository)
      )
    }
  }
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(list);
  config.service(create);
}
