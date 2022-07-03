use ntex::web;
use diesel::prelude::*;

use crate::{
  models::{Pool, CargoEnvPartial, CargoEnvItem, PgDeleteGeneric},
  controllers::errors::HttpError,
  repositories::errors::db_blocking_error,
  services,
};

// May be needed later
pub async fn _create(
  item: CargoEnvPartial,
  pool: &web::types::State<Pool>,
) -> Result<CargoEnvItem, HttpError> {
  use crate::schema::cargo_environnements::dsl;

  let conn = services::postgresql::get_pool_conn(pool)?;
  let res = web::block(move || {
    let item = CargoEnvItem {
      key: item.cargo_key.to_owned() + "-" + &item.name,
      cargo_key: item.cargo_key,
      name: item.name,
      value: item.value,
    };
    diesel::insert_into(dsl::cargo_environnements)
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

pub async fn create_many(
  items: Vec<CargoEnvPartial>,
  pool: &web::types::State<Pool>,
) -> Result<Vec<CargoEnvItem>, HttpError> {
  use crate::schema::cargo_environnements::dsl;

  let conn = services::postgresql::get_pool_conn(pool)?;
  let res = web::block(move || {
    let records = items
      .into_iter()
      .map(|item| CargoEnvItem {
        key: item.cargo_key.to_owned() + "-" + &item.name,
        cargo_key: item.cargo_key,
        name: item.name,
        value: item.value,
      })
      .collect::<Vec<CargoEnvItem>>();

    diesel::insert_into(dsl::cargo_environnements)
      .values(&records)
      .execute(&conn)?;
    Ok(records)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(items) => Ok(items),
  }
}

// May be needed later
pub async fn _delete_by_key(
  key: String,
  pool: &web::types::State<Pool>,
) -> Result<PgDeleteGeneric, HttpError> {
  use crate::schema::cargo_environnements::dsl;

  let conn = services::postgresql::get_pool_conn(pool)?;
  let res = web::block(move || {
    diesel::delete(dsl::cargo_environnements.filter(dsl::key.eq(key)))
      .execute(&conn)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(result) => Ok(PgDeleteGeneric { count: result }),
  }
}

pub async fn delete_by_cargo_key(
  cargo_key: String,
  pool: &web::types::State<Pool>,
) -> Result<PgDeleteGeneric, HttpError> {
  use crate::schema::cargo_environnements::dsl;

  let conn = services::postgresql::get_pool_conn(pool)?;
  let res = web::block(move || {
    diesel::delete(
      dsl::cargo_environnements.filter(dsl::cargo_key.eq(cargo_key)),
    )
    .execute(&conn)
  })
  .await;
  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(result) => Ok(PgDeleteGeneric { count: result }),
  }
}

pub async fn list_by_cargo_key(
  cargo_key: String,
  pool: &web::types::State<Pool>,
) -> Result<Vec<CargoEnvItem>, HttpError> {
  use crate::schema::cargo_environnements::dsl;

  let conn = services::postgresql::get_pool_conn(pool)?;
  let res = web::block(move || {
    dsl::cargo_environnements
      .filter(dsl::cargo_key.eq(cargo_key))
      .get_results(&conn)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(items) => Ok(items),
  }
}
