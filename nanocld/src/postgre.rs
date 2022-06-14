use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;

use crate::models::Pool;

/// Create postgres pool
///
/// # Examples
/// ```
/// let pool = create_pool();
/// ```
pub fn create_pool() -> Pool {
  let db_url = "postgres://root:root@nanocl-db-postgre/nanocl";
  let manager = ConnectionManager::<PgConnection>::new(db_url);

  r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create pool.")
}
