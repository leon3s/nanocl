use ntex::web;
use diesel::prelude::*;

use crate::models::{
  Pool, CargoProxyConfigItem, CargoProxyConfigPartial, PgDeleteGeneric,
};

use crate::controllers::errors::HttpError;
use crate::repositories::errors::db_blocking_error;
use crate::utils::get_pool_conn;

pub async fn get_for_cargo(
  cargo_key: String,
  pool: &web::types::State<Pool>,
) -> Result<CargoProxyConfigItem, HttpError> {
  use crate::schema::cargo_proxy_configs::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    dsl::cargo_proxy_configs
      .filter(dsl::cargo_key.eq(cargo_key))
      .get_result(&conn)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(item) => Ok(item),
  }
}

pub async fn create_for_cargo(
  cargo_key: String,
  item: CargoProxyConfigPartial,
  pool: &web::types::State<Pool>,
) -> Result<CargoProxyConfigItem, HttpError> {
  use crate::schema::cargo_proxy_configs::dsl;

  let conn = get_pool_conn(pool)?;

  let res = web::block(move || {
    let item = CargoProxyConfigItem {
      cargo_key,
      domain_name: item.domain_name,
      host_ip: item.host_ip,
    };
    diesel::insert_into(dsl::cargo_proxy_configs)
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

pub async fn delete_for_cargo(
  cargo_key: String,
  pool: &web::types::State<Pool>,
) -> Result<PgDeleteGeneric, HttpError> {
  use crate::schema::cargo_proxy_configs::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    diesel::delete(
      dsl::cargo_proxy_configs.filter(dsl::cargo_key.eq(cargo_key)),
    )
    .execute(&conn)
  })
  .await;
  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(result) => Ok(PgDeleteGeneric { count: result }),
  }
}
