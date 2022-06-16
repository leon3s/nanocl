//! Repository to manage namespaces in database
//! We can create delete list or inspect a namespace
use ntex::web;
use diesel::prelude::*;

use crate::utils::get_pool_conn;
use crate::controllers::errors::HttpError;
use crate::models::{NamespacePartial, NamespaceItem, PgDeleteGeneric, Pool};

use super::errors::db_blocking_error;

/// Create new namespace
///
/// # Arguments
///
/// * `item` - Partial namespace
/// * `pool` - Posgresql database pool
///
/// # Examples
///
/// ```rust,noerun
///
/// use crate::repositories;
///
/// let new_namespace = NamespaceCreate {
///   name: String::from("new-nsp"),
/// };
/// repositories::namespace::create(new_namespace, &pool).await;
/// ```
pub async fn create(
  item: NamespacePartial,
  pool: &web::types::State<Pool>,
) -> Result<NamespaceItem, HttpError> {
  use crate::schema::namespaces::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    let item = NamespaceItem { name: item.name };
    diesel::insert_into(dsl::namespaces)
      .values(&item)
      .execute(&conn)?;
    Ok(item)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(item) => Ok(item),
  }
}

/// List all namespace
///
/// # Arguments
///
/// * `pool` - Posgresql database pool
///
/// # Examples
///
/// ```
///
/// use crate::repositories;
/// repositories::namespace::list(&pool).await;
/// ```
pub async fn list(
  pool: &web::types::State<Pool>,
) -> Result<Vec<NamespaceItem>, HttpError> {
  use crate::schema::namespaces::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || dsl::namespaces.load(&conn)).await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(items) => Ok(items),
  }
}

/// Inspect namespace by id or name
///
/// # Arguments
///
/// * `id_or_name` Id or name of the namespace
/// * `pool` - Posgresql database pool
///
/// # Examples
///
/// ```rust,norun
/// use crate::repositories;
///
/// repositories::namespace::inspect_name(String::from("default"), &pool).await;
/// ```
pub async fn inspect_name(
  name: String,
  pool: &web::types::State<Pool>,
) -> Result<NamespaceItem, HttpError> {
  use crate::schema::namespaces::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    dsl::namespaces.filter(dsl::name.eq(name)).get_result(&conn)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(item) => Ok(item),
  }
}

/// Delete namespace by id or name
///
/// # Arguments
///
/// * `id_or_name` Id or name of the namespace
/// * `pool` - Posgresql database pool
///
/// # Examples
///
/// ```rust,norun
/// use crate::repositories;
///
/// repositories::namespace::delete_by_name(String::from("default"), &pool).await;
/// ```
pub async fn delete_by_name(
  name: String,
  pool: &web::types::State<Pool>,
) -> Result<PgDeleteGeneric, HttpError> {
  use crate::schema::namespaces::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    diesel::delete(dsl::namespaces.filter(dsl::name.eq(name))).execute(&conn)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(result) => Ok(PgDeleteGeneric { count: result }),
  }
}

#[cfg(test)]
mod test_namespace {
  use super::*;
  use crate::postgre;

  #[ntex::test]
  async fn main() -> Result<(), HttpError> {
    let pool = postgre::create_pool();
    let pool_state = web::types::State::new(pool);

    // List namespace
    let res = list(&pool_state).await?;
    assert!(res.is_empty());
    let namespace_name = String::from("test-default");
    let item = NamespacePartial {
      name: namespace_name.clone(),
    };

    // Create namespace
    let res = create(item, &pool_state).await?;
    assert_eq!(res.name, namespace_name.clone());

    // Inspect namespace
    let res = inspect_name(namespace_name.clone(), &pool_state).await?;
    assert_eq!(res.name, namespace_name.clone());

    // Delete namespace
    let res = delete_by_name(namespace_name.clone(), &pool_state).await?;
    assert_eq!(res.count, 1);

    Ok(())
  }
}
