//! Repository to manage namespaces in database
//! We can create delete list or inspect a namespace
use ntex::web;
use diesel::prelude::*;

use crate::services;
use crate::models::{Pool, NamespacePartial, NamespaceItem, PgDeleteGeneric};

use crate::errors::HttpResponseError;
use super::errors::db_blocking_error;

/// Create new namespace
///
/// # Arguments
///
/// * [name](String) - Partial namespace
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
) -> Result<NamespaceItem, HttpResponseError> {
  use crate::schema::namespaces::dsl;

  let conn = services::postgresql::get_pool_conn(pool)?;
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
) -> Result<Vec<NamespaceItem>, HttpResponseError> {
  use crate::schema::namespaces::dsl;

  let conn = services::postgresql::get_pool_conn(pool)?;
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
/// repositories::namespace::inspect_by_name(String::from("default"), &pool).await;
/// ```
pub async fn inspect_by_name(
  name: String,
  pool: &web::types::State<Pool>,
) -> Result<NamespaceItem, HttpResponseError> {
  use crate::schema::namespaces::dsl;

  let conn = services::postgresql::get_pool_conn(pool)?;
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
) -> Result<PgDeleteGeneric, HttpResponseError> {
  use crate::schema::namespaces::dsl;

  let conn = services::postgresql::get_pool_conn(pool)?;
  let res = web::block(move || {
    diesel::delete(dsl::namespaces.filter(dsl::name.eq(name))).execute(&conn)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(result) => Ok(PgDeleteGeneric { count: result }),
  }
}

pub async fn find_by_name(
  name: String,
  pool: &web::types::State<Pool>,
) -> Result<NamespaceItem, HttpResponseError> {
  use crate::schema::namespaces::dsl;

  let conn = services::postgresql::get_pool_conn(pool)?;
  let res = web::block(move || {
    dsl::namespaces.filter(dsl::name.eq(name)).get_result(&conn)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(item) => Ok(item),
  }
}

#[cfg(test)]
mod test_namespace {
  use super::*;

  use crate::utils::test::*;
  #[ntex::test]
  async fn main() -> Result<(), HttpResponseError> {
    let pool = gen_postgre_pool().await;
    let pool_state = web::types::State::new(pool);

    // List namespace
    let _res = list(&pool_state).await?;
    let namespace_name = String::from("test-default");
    let item = NamespacePartial {
      name: namespace_name.clone(),
    };

    // Create namespace
    let res = create(item, &pool_state).await?;
    assert_eq!(res.name, namespace_name.clone());

    // Inspect namespace
    let res = inspect_by_name(namespace_name.clone(), &pool_state).await?;
    assert_eq!(res.name, namespace_name.clone());

    // Delete namespace
    let res = delete_by_name(namespace_name.clone(), &pool_state).await?;
    assert_eq!(res.count, 1);

    Ok(())
  }
}
