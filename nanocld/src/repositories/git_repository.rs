use ntex::web;
use diesel::prelude::*;

use crate::utils::get_pool_conn;
use crate::controllers::errors::HttpError;
use crate::models::{
  PgDeleteGeneric, Pool, GitRepositoryPartial, GitRepositoryItem,
  GitRepositorySourceType,
};

use super::errors::db_blocking_error;

/// Create fresh git repository
///
/// # Arguments
///
/// * `item` - Partial GitRepository
/// * `pool` - Posgresql database pool
///
/// # Examples
///
/// ```
///
/// use crate::repositories::git_repository;
/// let new_repository = GitRepositoryItem {}
///
/// git_repository::create(new_branches, &pool).await;
/// ```
pub async fn create(
  item: GitRepositoryPartial,
  default_branch: String,
  pool: &web::types::State<Pool>,
) -> Result<GitRepositoryItem, HttpError> {
  use crate::schema::git_repositories::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    let new_namespace = GitRepositoryItem {
      url: item.url,
      name: item.name,
      default_branch,
      source: GitRepositorySourceType::Github,
    };
    diesel::insert_into(dsl::git_repositories)
      .values(&new_namespace)
      .execute(&conn)?;
    Ok(new_namespace)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(item) => Ok(item),
  }
}

/// Delete git repository by id or name
///
/// # Arguments
///
/// * `id` - Id or name of git repository
/// * `pool` - Posgresql database pool
///
/// # Examples
///
/// ```
///
/// use crate::repositories::git_repository;
///
/// git_repository::delete_by_name(id, &pool).await;
/// ```
pub async fn delete_by_name(
  id: String,
  pool: &web::types::State<Pool>,
) -> Result<PgDeleteGeneric, HttpError> {
  use crate::schema::git_repositories::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    diesel::delete(dsl::git_repositories.filter(dsl::name.eq(id)))
      .execute(&conn)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(result) => Ok(PgDeleteGeneric { count: result }),
  }
}

/// Find git repository by his id or name
///
/// # Arguments
///
/// * `id` - Id or name of git repository
/// * `pool` - Posgresql database pool
///
/// # Examples
///
/// ```
///
/// use crate::repositories::git_repository;
///
/// git_repository::find_by_name(id, &pool).await;
/// ```
pub async fn find_by_name(
  id_or_name: String,
  pool: &web::types::State<Pool>,
) -> Result<GitRepositoryItem, HttpError> {
  use crate::schema::git_repositories::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    dsl::git_repositories
      .filter(dsl::name.eq(id_or_name))
      .get_result(&conn)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(item) => Ok(item),
  }
}

/// List all git repository
///
/// # Arguments
///
/// * `pool` - Posgresql database pool
///
/// # Examples
///
/// ```
///
/// use crate::repositories::git_repository;
///
/// git_repository::list(id, &pool).await;
/// ```
pub async fn list(
  pool: &web::types::State<Pool>,
) -> Result<Vec<GitRepositoryItem>, HttpError> {
  use crate::schema::git_repositories::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || dsl::git_repositories.load(&conn)).await;
  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(items) => Ok(items),
  }
}

#[cfg(test)]
mod test_git_repository {
  use super::*;

  use crate::utils::test::*;

  #[ntex::test]
  async fn main() {
    let pool = gen_postgre_pool().await;
    let pool_state = web::types::State::new(pool);
    // Find
    let _res = list(&pool_state).await.unwrap();
    let item = GitRepositoryPartial {
      name: String::from("test"),
      url: String::from("https://github.com/leon3s/express-test-deploy"),
    };
    // Create
    let res = create(item, String::from("development"), &pool_state)
      .await
      .unwrap();
    assert_eq!(res.name, "test");

    // Find by name
    let res = find_by_name(res.name, &pool_state).await.unwrap();
    assert_eq!(res.name, "test");
    assert_eq!(res.name, "test");

    // Delete with name
    let res = delete_by_name(res.name.to_string(), &pool_state)
      .await
      .unwrap();
    assert_eq!(res.count, 1);
  }
}
