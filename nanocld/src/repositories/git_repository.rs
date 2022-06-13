use diesel::prelude::*;
use ntex::web;
use uuid::Uuid;

use crate::controllers::errors::HttpError;
use crate::models::{
  GitRepositoryCreate, GitRepositoryItem, GitRepositorySourceType,
  PgDeleteGeneric, Pool,
};
use crate::repositories::errors::db_blocking_error;
use crate::utils::get_pool_conn;

/// Create git repository
pub async fn create(
  item: GitRepositoryCreate,
  pool: &web::types::State<Pool>,
) -> Result<GitRepositoryItem, HttpError> {
  use crate::schema::git_repositories::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    let new_namespace = GitRepositoryItem {
      url: item.url,
      id: Uuid::new_v4(),
      name: item.name,
      token: item.token,
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
pub async fn delete_by_id_or_name(
  id: String,
  pool: &web::types::State<Pool>,
) -> Result<PgDeleteGeneric, HttpError> {
  use crate::schema::git_repositories::dsl;

  let conn = get_pool_conn(pool)?;
  let res = match Uuid::parse_str(&id) {
    Err(_) => {
      web::block(move || {
        diesel::delete(dsl::git_repositories.filter(dsl::name.eq(id)))
          .execute(&conn)
      })
      .await
    }
    Ok(uuid) => {
      web::block(move || {
        diesel::delete(dsl::git_repositories.filter(dsl::id.eq(uuid)))
          .execute(&conn)
      })
      .await
    }
  };

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(result) => Ok(PgDeleteGeneric { count: result }),
  }
}

pub async fn find_by_id_or_name(
  id_or_name: String,
  pool: &web::types::State<Pool>,
) -> Result<GitRepositoryItem, HttpError> {
  use crate::schema::git_repositories::dsl;

  let conn = get_pool_conn(pool)?;
  let res = match Uuid::parse_str(&id_or_name) {
    Err(_) => {
      web::block(move || {
        dsl::git_repositories
          .filter(dsl::name.eq(id_or_name))
          .get_result(&conn)
      })
      .await
    }
    Ok(uuid) => {
      web::block(move || {
        dsl::git_repositories
          .filter(dsl::id.eq(uuid))
          .get_result(&conn)
      })
      .await
    }
  };

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(item) => Ok(item),
  }
}

/// List all git repository
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
  use crate::postgre;

  use super::*;

  #[ntex::test]
  async fn main() {
    let pool = postgre::create_pool();
    let pool_state = web::types::State::new(pool);
    // Find
    let _res = list(&pool_state).await.unwrap();
    let item = GitRepositoryCreate {
      token: None,
      name: String::from("test"),
      url: String::from("https://github.com/leon3s/express-test-deploy"),
    };
    // Create
    let res = create(item, &pool_state).await.unwrap();
    assert_eq!(res.name, "test");

    // Find by id or name
    let res = find_by_id_or_name(res.name, &pool_state).await.unwrap();
    assert_eq!(res.name, "test");
    let res = find_by_id_or_name(res.id.to_string(), &pool_state)
      .await
      .unwrap();
    assert_eq!(res.name, "test");

    // Delete with id
    let res = delete_by_id_or_name(res.id.to_string(), &pool_state)
      .await
      .unwrap();
    assert_eq!(res.count, 1);
    let item = GitRepositoryCreate {
      name: String::from("test"),
      token: Some(String::from("test")),
      url: String::from("https://github.com/leon3s/express-test-deploy"),
    };
    let res = create(item, &pool_state).await.unwrap();
    let res = delete_by_id_or_name(res.name, &pool_state).await.unwrap();
    assert_eq!(res.count, 1);
  }
}
