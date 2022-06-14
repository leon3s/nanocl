use ntex::web;
use uuid::Uuid;
use diesel::prelude::*;

use crate::models::Pool;
use crate::utils::get_pool_conn;
use crate::controllers::errors::HttpError;
use crate::models::{
  GitRepositoryBranchCreate, GitRepositoryBranchItem, PgDeleteGeneric,
};

use super::errors::db_blocking_error;

/// Create multiple git repository branch
///
/// # Arguments
///
/// * `items` - Partial GitRepositoryBranch
/// * `pool` - Posgresql database pool
///
/// # Examples
///
/// ```
///
/// use crate::repositories::git_repository_branch;
/// let new_branches = vec![
///   GitRepositoryBranchCreate {}
/// ]
/// git_repository_branch::create_many(new_branches, pool).await;
/// ```
pub async fn create_many(
  items: Vec<GitRepositoryBranchCreate>,
  pool: &web::types::State<Pool>,
) -> Result<Vec<GitRepositoryBranchItem>, HttpError> {
  use crate::schema::git_repository_branches::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    let branches = items
      .into_iter()
      .map(|item| GitRepositoryBranchItem {
        id: Uuid::new_v4(),
        name: item.name,
        repository_id: item.repository_id,
      })
      .collect::<Vec<GitRepositoryBranchItem>>();
    diesel::insert_into(dsl::git_repository_branches)
      .values(&branches)
      .execute(&conn)?;
    Ok(branches)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(branches) => Ok(branches),
  }
}

/// Delete all branches for given repository id and return number of deleted entry
///
/// # Arguments
///
/// * `repository_id` - Git repository id
/// * `pool` - Posgresql database pool
///
/// # Examples
///
/// ```
///
/// use crate::repositories::git_repository_branch;
/// git_repository_branch::delete_by_repository_id(repository_id, pool).await;
/// ```
pub async fn delete_by_repository_id(
  repository_id: Uuid,
  pool: &web::types::State<Pool>,
) -> Result<PgDeleteGeneric, HttpError> {
  use crate::schema::git_repository_branches::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    diesel::delete(dsl::git_repository_branches)
      .filter(dsl::repository_id.eq(repository_id))
      .execute(&conn)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(result) => Ok(PgDeleteGeneric { count: result }),
  }
}

#[cfg(test)]
mod test {
  use crate::postgre;
  use crate::repositories::git_repository;
  use crate::models::{GitRepositoryCreate, GitRepositoryBranchCreate};

  use crate::utils::test::*;

  use super::*;

  #[ntex::test]
  async fn main() -> TestReturn {
    let pool = postgre::create_pool();
    let pool_state = web::types::State::new(pool);

    let new_repository = GitRepositoryCreate {
      name: String::from("test-branch"),
      url: String::from("test"),
      token: None,
    };
    let res = git_repository::create(new_repository, &pool_state)
      .await
      .unwrap();

    // Create many branches
    let items = vec![GitRepositoryBranchCreate {
      name: String::from("test-branch"),
      repository_id: res.id.clone(),
    }];
    create_many(items, &pool_state).await.unwrap();

    // Delete branch by repository id
    delete_by_repository_id(res.id, &pool_state).await.unwrap();

    git_repository::delete_by_id_or_name(
      String::from("test-branch"),
      &pool_state,
    )
    .await
    .unwrap();
    // todo
    Ok(())
  }
}
