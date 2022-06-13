use diesel::prelude::*;
/// Repository to manage namespaces in database
/// We can create delete list or inspect a namespace
use ntex::web;
use uuid::Uuid;

use crate::controllers::errors::HttpError;
use crate::models::{NamespaceCreate, NamespaceItem, PgDeleteGeneric, Pool};
use crate::utils::get_pool_conn;

use super::errors::db_blocking_error;

/// Create a fresh namespace
pub async fn create(
  item: NamespaceCreate,
  pool: &web::types::State<Pool>,
) -> Result<NamespaceItem, HttpError> {
  use crate::schema::namespaces::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    let item = NamespaceItem {
      id: Uuid::new_v4(),
      name: item.name,
    };
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

/// Inspect a namespace by it's name or id
pub async fn inspect_by_id_or_name(
  id_or_name: String,
  pool: &web::types::State<Pool>,
) -> Result<NamespaceItem, HttpError> {
  use crate::schema::namespaces::dsl;

  let conn = get_pool_conn(pool)?;
  let res = match Uuid::parse_str(&id_or_name) {
    Err(_) => {
      web::block(move || {
        dsl::namespaces
          .filter(dsl::name.eq(id_or_name))
          .get_result(&conn)
      })
      .await
    }
    Ok(uuid) => {
      web::block(move || {
        dsl::namespaces.filter(dsl::id.eq(uuid)).get_result(&conn)
      })
      .await
    }
  };

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(item) => Ok(item),
  }
}

/// Delete a namespace by it's name or id
pub async fn delete_by_id_or_name(
  id_or_name: String,
  pool: &web::types::State<Pool>,
) -> Result<PgDeleteGeneric, HttpError> {
  use crate::schema::namespaces::dsl;

  let conn = get_pool_conn(pool)?;
  let res = match Uuid::parse_str(&id_or_name) {
    Err(_) => {
      web::block(move || {
        diesel::delete(dsl::namespaces.filter(dsl::name.eq(id_or_name)))
          .execute(&conn)
      })
      .await
    }
    Ok(uuid) => {
      web::block(move || {
        diesel::delete(dsl::namespaces.filter(dsl::id.eq(uuid))).execute(&conn)
      })
      .await
    }
  };

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
    let namespace_name = String::from("default");
    let item = NamespaceCreate {
      name: namespace_name.clone(),
    };
    // Create namespace
    let res = create(item, &pool_state).await?;
    assert_eq!(res.name, namespace_name.clone());
    // Inspect namespace
    let res =
      inspect_by_id_or_name(namespace_name.clone(), &pool_state).await?;
    assert_eq!(res.name, namespace_name.clone());
    // Delete namespace
    let res = delete_by_id_or_name(namespace_name.clone(), &pool_state).await?;
    assert_eq!(res.count, 1);
    Ok(())
  }
}
