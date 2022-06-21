use ntex::web;
use diesel::prelude::*;

use crate::utils::get_pool_conn;
use crate::controllers::errors::HttpError;
use crate::models::{Pool, CargoItem, CargoPartial, PgDeleteGeneric, NamespaceItem};

use super::errors::db_blocking_error;

pub async fn find_by_namespace(
  nsp: NamespaceItem,
  pool: &web::types::State<Pool>,
) -> Result<Vec<CargoItem>, HttpError> {
  let conn = get_pool_conn(pool)?;

  let res = web::block(move || CargoItem::belonging_to(&nsp).load(&conn)).await;
  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(items) => Ok(items),
  }
}

pub async fn create(
  nsp: String,
  item: CargoPartial,
  pool: &web::types::State<Pool>,
) -> Result<CargoItem, HttpError> {
  use crate::schema::cargos::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    let new_item = CargoItem {
      key: nsp.to_owned() + "-" + &item.name,
      name: item.name.clone(),
      namespace_name: nsp,
      image_name: item.image_name,
      network_name: item.network_name.unwrap_or_else(|| String::from("")),
    };
    diesel::insert_into(dsl::cargos)
      .values(&new_item)
      .execute(&conn)?;
    Ok(new_item)
  })
  .await;
  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(item) => Ok(item),
  }
}

pub async fn delete_by_key(
  key: String,
  pool: &web::types::State<Pool>,
) -> Result<PgDeleteGeneric, HttpError> {
  use crate::schema::cargos::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    diesel::delete(dsl::cargos)
      .filter(dsl::key.eq(key))
      .execute(&conn)
  })
  .await;
  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(result) => Ok(PgDeleteGeneric { count: result }),
  }
}

pub async fn find_by_key(
  key: String,
  pool: &web::types::State<Pool>,
) -> Result<CargoItem, HttpError> {
  use crate::schema::cargos::dsl;

  let conn = get_pool_conn(pool)?;
  let res =
    web::block(move || dsl::cargos.filter(dsl::key.eq(key)).get_result(&conn))
      .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(item) => Ok(item),
  }
}
