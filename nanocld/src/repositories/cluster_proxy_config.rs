use ntex::web;
use diesel::prelude::*;

use crate::services;
use crate::models::{
  Pool, ClusterProxyConfigItem, ClusterProxyConfigPartial, PgDeleteGeneric,
};

use crate::errors::HttpResponseError;
use super::errors::db_blocking_error;

pub async fn get_for_cluster(
  cluster_key: String,
  pool: &web::types::State<Pool>,
) -> Result<ClusterProxyConfigItem, HttpResponseError> {
  use crate::schema::cluster_proxy_configs::dsl;

  let conn = services::postgresql::get_pool_conn(pool)?;
  let res = web::block(move || {
    dsl::cluster_proxy_configs
      .filter(dsl::cluster_key.eq(cluster_key))
      .get_result(&conn)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(item) => Ok(item),
  }
}

pub async fn create_for_cluster(
  cluster_key: String,
  item: ClusterProxyConfigPartial,
  pool: &web::types::State<Pool>,
) -> Result<ClusterProxyConfigItem, HttpResponseError> {
  use crate::schema::cluster_proxy_configs::dsl;

  let conn = services::postgresql::get_pool_conn(pool)?;

  let res = web::block(move || {
    let item = ClusterProxyConfigItem {
      cluster_key,
      template: item.template,
      target_port: item.target_port,
    };
    diesel::insert_into(dsl::cluster_proxy_configs)
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

pub async fn delete_for_cluster(
  cluster_key: String,
  pool: &web::types::State<Pool>,
) -> Result<PgDeleteGeneric, HttpResponseError> {
  use crate::schema::cluster_proxy_configs::dsl;

  let conn = services::postgresql::get_pool_conn(pool)?;
  let res = web::block(move || {
    diesel::delete(
      dsl::cluster_proxy_configs.filter(dsl::cluster_key.eq(cluster_key)),
    )
    .execute(&conn)
  })
  .await;
  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(result) => Ok(PgDeleteGeneric { count: result }),
  }
}
