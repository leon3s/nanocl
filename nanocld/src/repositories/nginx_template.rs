use ntex::web;
use diesel::prelude::*;

use crate::services;
use crate::models::{Pool, NginxTemplateItem, PgDeleteGeneric};

use crate::controllers::errors::HttpError;
use crate::repositories::errors::db_blocking_error;

pub async fn list(
  pool: &web::types::State<Pool>,
) -> Result<Vec<NginxTemplateItem>, HttpError> {
  use crate::schema::nginx_templates::dsl;

  let conn = services::postgresql::get_pool_conn(pool)?;
  let res = web::block(move || dsl::nginx_templates.load(&conn)).await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(items) => Ok(items),
  }
}

pub async fn create(
  item: NginxTemplateItem,
  pool: &web::types::State<Pool>,
) -> Result<NginxTemplateItem, HttpError> {
  use crate::schema::nginx_templates::dsl;

  let conn = services::postgresql::get_pool_conn(pool)?;
  let res = web::block(move || {
    diesel::insert_into(dsl::nginx_templates)
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

pub async fn get_by_name(
  name: String,
  pool: &web::types::State<Pool>,
) -> Result<NginxTemplateItem, HttpError> {
  use crate::schema::nginx_templates::dsl;

  let conn = services::postgresql::get_pool_conn(pool)?;
  let res = web::block(move || {
    dsl::nginx_templates
      .filter(dsl::name.eq(name))
      .get_result(&conn)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(item) => Ok(item),
  }
}

pub async fn delete_by_name(
  name: String,
  pool: &web::types::State<Pool>,
) -> Result<PgDeleteGeneric, HttpError> {
  use crate::schema::nginx_templates::dsl;

  let conn = services::postgresql::get_pool_conn(pool)?;
  let res = web::block(move || {
    diesel::delete(dsl::nginx_templates.filter(dsl::name.eq(name)))
      .execute(&conn)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(result) => Ok(PgDeleteGeneric { count: result }),
  }
}
