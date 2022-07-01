use ntex::web;
use diesel::prelude::*;

use crate::models::{Pool, ClusterCargoPartial, ClusterCargoItem, PgDeleteGeneric};

use crate::controllers::errors::HttpError;
use crate::repositories::errors::db_blocking_error;
use crate::utils::get_pool_conn;

pub async fn create(
  item: ClusterCargoPartial,
  pool: &web::types::State<Pool>,
) -> Result<ClusterCargoItem, HttpError> {
  use crate::schema::cluster_cargoes::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    let item = ClusterCargoItem {
      key: format!("{}-{}", item.cluster_key, item.cargo_key),
      network_key: item.network_key,
      cluster_key: item.cluster_key,
      cargo_key: item.cargo_key,
    };
    diesel::insert_into(dsl::cluster_cargoes)
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

pub async fn get_by_cluster_key(
  cluster_key: String,
  pool: &web::types::State<Pool>,
) -> Result<Vec<ClusterCargoItem>, HttpError> {
  use crate::schema::cluster_cargoes::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    dsl::cluster_cargoes
      .filter(dsl::cluster_key.eq(cluster_key))
      .get_results(&conn)
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
  use crate::schema::cluster_cargoes::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    diesel::delete(dsl::cluster_cargoes.filter(dsl::key.eq(key))).execute(&conn)
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
  use crate::schema::cluster_cargoes::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    diesel::delete(dsl::cluster_cargoes.filter(dsl::cargo_key.eq(cargo_key)))
      .execute(&conn)
  })
  .await;
  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(result) => Ok(PgDeleteGeneric { count: result }),
  }
}
