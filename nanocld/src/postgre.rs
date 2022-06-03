use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

use crate::models::Pool;

pub fn create_pool() -> Pool {
    let db_url = "postgres://root:root@nanocl-db-postgre/nanocl";
    let manager = ConnectionManager::<PgConnection>::new(db_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
