use uuid::Uuid;
use utoipa::Component;
use serde::{Serialize, Deserialize};
use r2d2::PooledConnection;
use diesel::{r2d2::ConnectionManager, PgConnection};

use crate::schema::namespaces;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DBConn = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Component, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="namespaces"]
pub struct NamespaceItem {
  pub(crate) id: Uuid,
  pub(crate) name: String,
}

#[derive(Component, Deserialize)]
pub struct NamespaceCreate {
  pub(crate) name: String,
}
