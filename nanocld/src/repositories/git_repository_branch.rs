use ntex::web;
use uuid::Uuid;
use diesel::prelude::*;

use crate::models::{
  PgDeleteGeneric,
  GitRepositoryBranchItem,
  GitRepositoryBranchCreate,
};

use crate::models::Pool;
use crate::utils::get_pool_conn;
use crate::controllers::errors::HttpError;

use super::errors::db_blocking_error;

pub async fn create_many(
  items: Vec<GitRepositoryBranchCreate>,
  pool: &web::types::State<Pool>,
) -> Result<Vec<GitRepositoryBranchItem>, HttpError>{
  use crate::schema::git_repository_branches::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    let branches = items.into_iter().map(|item| {
      GitRepositoryBranchItem {
        id: Uuid::new_v4(),
        name: item.name,
        repository_id: item.repository_id,
      }
    }).collect::<Vec<GitRepositoryBranchItem>>();
    diesel::insert_into(dsl::git_repository_branches).values(&branches).execute(&conn)?;
    Ok(branches)
  }).await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(branches) => Ok(branches),
  }
}

pub async fn delete_by_repository_id(
  repository_id: Uuid,
  pool: &web::types::State<Pool>,
) -> Result<PgDeleteGeneric, HttpError>{
  use crate::schema::git_repository_branches::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    diesel::delete(dsl::git_repository_branches)
    .filter(dsl::repository_id.eq(repository_id))
    .execute(&conn)
  }).await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(result) => Ok(PgDeleteGeneric {
      count: result
    })
  }
}
