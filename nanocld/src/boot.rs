//! File used to describe daemon boot
use ntex::web;

use crate::postgre;
use crate::repositories;
use crate::controllers::errors::HttpError;
use crate::models::{Pool, NamespacePartial};

#[derive(Debug)]
pub enum BootError {
  Errorhttp(HttpError),
}

#[derive(Clone)]
pub struct DaemonState {
  pub(crate) pool: Pool,
}

/// # Create default namespace
/// Create a namespace with default as name if he doesn't exist
///
/// # Arguments
/// - [pool](web::types::State<Pool>) Postgres database pool
///
/// # Examples
/// ```rust,norun
/// create_default_nsp(&pool).await;
/// ```
async fn create_default_nsp(
  pool: &web::types::State<Pool>,
) -> Result<(), BootError> {
  const NSP_NAME: &str = "default";
  match repositories::namespace::inspect_by_id_or_name(
    NSP_NAME.to_string(),
    pool,
  )
  .await
  {
    Err(_err) => {
      let new_nsp = NamespacePartial {
        name: NSP_NAME.to_string(),
      };
      match repositories::namespace::create(new_nsp, pool).await {
        Err(err) => Err(BootError::Errorhttp(err)),
        Ok(_nsp) => Ok(()),
      }
    }
    Ok(_) => Ok(()),
  }
}

/// Boot function called before server start to initialize his state
pub async fn boot() -> Result<DaemonState, BootError> {
  let db_pool = postgre::create_pool();
  let pool = web::types::State::new(db_pool.clone());

  create_default_nsp(&pool).await?;

  Ok(DaemonState { pool: db_pool })
}
