//! Repository to manage nginx_logs in database
use ntex::web;
use diesel::prelude::*;

use crate::services;
use crate::models::{Pool, NginxLogItem, NginxLogPartial};

use crate::errors::HttpResponseError;
use super::errors::db_blocking_error;

pub async fn create_log(
  partial: NginxLogPartial,
  pool: &web::types::State<Pool>,
) -> Result<NginxLogItem, HttpResponseError> {
  use crate::schema::nginx_logs::dsl;

  let conn = services::postgresql::get_pool_conn(pool)?;
  let res = web::block(move || {
    let item = NginxLogItem::from(partial);
    diesel::insert_into(dsl::nginx_logs)
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
