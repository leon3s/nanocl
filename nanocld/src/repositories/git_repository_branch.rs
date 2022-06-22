use ntex::web;
use diesel::prelude::*;

use crate::models::Pool;
use crate::utils::get_pool_conn;
use crate::controllers::errors::HttpError;
use crate::models::{
  GitRepositoryBranchPartial, GitRepositoryBranchItem, PgDeleteGeneric,
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
  items: Vec<GitRepositoryBranchPartial>,
  pool: &web::types::State<Pool>,
) -> Result<Vec<GitRepositoryBranchItem>, HttpError> {
  use crate::schema::git_repository_branches::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    let branches = items
      .into_iter()
      .map(|item| GitRepositoryBranchItem {
        key: item.repository_name.to_owned() + "-" + &item.name,
        name: item.name,
        last_commit_sha: item.last_commit_sha,
        repository_name: item.repository_name,
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

/// - Delete all branches for given repository id and return number of deleted entry
///
/// # Arguments
///
/// * `repository_id` - Git repository id
/// * `pool` - Posgresql database pool
///
/// # Examples
///
/// ```rust
/// use crate::repositories::git_repository_branch;
/// git_repository_branch::delete_by_repository_id(repository_id, pool).await;
/// ```
pub async fn delete_by_repository_id(
  repository_name: String,
  pool: &web::types::State<Pool>,
) -> Result<PgDeleteGeneric, HttpError> {
  use crate::schema::git_repository_branches::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    diesel::delete(dsl::git_repository_branches)
      .filter(dsl::repository_name.eq(repository_name))
      .execute(&conn)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(result) => Ok(PgDeleteGeneric { count: result }),
  }
}

pub async fn get_by_key(
  key: String,
  pool: &web::types::State<Pool>,
) -> Result<GitRepositoryBranchItem, HttpError> {
  use crate::schema::git_repository_branches::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    dsl::git_repository_branches
      .filter(dsl::key.eq(key))
      .get_result(&conn)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(item) => Ok(item),
  }
}

pub async fn update_item(
  item: GitRepositoryBranchItem,
  pool: &web::types::State<Pool>,
) -> Result<(), HttpError> {
  use crate::schema::git_repository_branches::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    diesel::update(dsl::git_repository_branches.filter(dsl::key.eq(item.key)))
      .set(dsl::last_commit_sha.eq(item.last_commit_sha))
      .execute(&conn)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(_) => Ok(()),
  }
}

#[cfg(test)]
mod test {
  use crate::postgre;
  use crate::repositories::git_repository;
  use crate::models::{GitRepositoryPartial, GitRepositoryBranchPartial};

  use crate::utils::test::*;

  use super::*;

  #[ntex::test]
  async fn main() -> TestReturn {
    let pool = postgre::create_pool();
    let pool_state = web::types::State::new(pool);

    let new_repository = GitRepositoryPartial {
      name: String::from("test-branch"),
      url: String::from("test"),
    };
    let res = git_repository::create(
      new_repository,
      String::from("master"),
      &pool_state,
    )
    .await
    .unwrap();

    // Create many branches
    let items = vec![GitRepositoryBranchPartial {
      name: String::from("test-branch"),
      last_commit_sha: String::from("sha256:super_commit!"),
      repository_name: res.name.to_owned(),
    }];
    create_many(items, &pool_state).await.unwrap();

    // Delete branch by repository id
    delete_by_repository_id(res.name.to_owned(), &pool_state)
      .await
      .unwrap();

    git_repository::delete_by_name(String::from("test-branch"), &pool_state)
      .await
      .unwrap();
    // todo
    Ok(())
  }
}
