use ntex::web;
use diesel::prelude::*;
use utoipa::openapi::security::Http;

use crate::controllers::errors::HttpError;
use crate::models::{
  Pool, CargoPortPartial, CargoPortItem, CargoItem, PgDeleteGeneric,
};
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

pub async fn update_many(
  items: Vec<CargoPortItem>,
  pool: &web::types::State<Pool>,
) -> Result<(), HttpError> {
  use crate::schema::cargo_ports::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    items.into_iter().try_for_each(
      |item| -> Result<(), diesel::result::Error> {
        diesel::update(dsl::cargo_ports.filter(dsl::key.eq(item.key)))
          .set((dsl::from.eq(item.from), dsl::to.eq(item.to)))
          .execute(&conn)?;
        Ok(())
      },
    )
  })
  .await;
  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(_) => Ok(()),
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
      key: cargo_key.to_owned() + &item.to.to_string(),
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

pub async fn delete_for_cargo(
  cargo_key: String,
  pool: &web::types::State<Pool>,
) -> Result<PgDeleteGeneric, HttpError> {
  use crate::schema::cargo_ports::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    diesel::delete(dsl::cargo_ports.filter(dsl::cargo_key.eq(cargo_key)))
      .execute(&conn)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(result) => Ok(PgDeleteGeneric { count: result }),
  }
}
