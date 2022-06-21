use ntex::web;
use diesel::prelude::*;

use crate::controllers::errors::HttpError;
use crate::models::{Pool, CargoPortPartial, CargoPortItem, CargoItem};
use crate::repositories::errors::db_blocking_error;
use crate::utils::get_pool_conn;

pub async fn create_many_for_cargo(
  cargo_key: String,
  items: Vec<CargoPortPartial>,
  pool: &web::types::State<Pool>,
) -> Result<Vec<CargoPortItem>, HttpError> {
  use crate::schema::cargo_ports::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    let items = items
      .into_iter()
      .map(|item| CargoPortItem {
        key: cargo_key.to_owned() + &item.from.to_string(),
        cargo_key: cargo_key.clone(),
        from: item.from,
        to: item.to,
      })
      .collect::<Vec<CargoPortItem>>();
    diesel::insert_into(dsl::cargo_ports)
      .values(&items)
      .execute(&conn)?;
    Ok(items)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(items) => Ok(items),
  }
}

/// May need only the create many
pub async fn _create_for_cargo(
  cargo_key: String,
  item: CargoPortPartial,
  pool: &web::types::State<Pool>,
) -> Result<CargoPortItem, HttpError> {
  use crate::schema::cargo_ports::dsl;

  let conn = get_pool_conn(pool)?;

  let res = web::block(move || {
    let item = CargoPortItem {
      key: cargo_key.to_owned() + &item.from.to_string(),
      cargo_key,
      from: item.from,
      to: item.to,
    };
    diesel::insert_into(dsl::cargo_ports)
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

pub async fn list_for_cargo(
  item: CargoItem,
  pool: &web::types::State<Pool>,
) -> Result<Vec<CargoPortItem>, HttpError> {
  let conn = get_pool_conn(pool)?;
  let res =
    web::block(move || CargoPortItem::belonging_to(&item).load(&conn)).await;
  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(items) => Ok(items),
  }
}