use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;

use crate::models::Pool;

/// # Create pool
/// Create an pool connection to postgres database
///
/// # Returns
/// - [Pool](Pool) R2d2 pool connection for postgres
///
/// # Examples
/// ```
/// let pool = create_pool();
/// ```
pub fn create_pool(host: String) -> Pool {
  let db_url = "postgres://root:root@".to_owned() + &host + "/nanocl";
  let manager = ConnectionManager::<PgConnection>::new(db_url);

  r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create pool.")
}
