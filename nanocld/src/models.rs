use uuid::Uuid;
use utoipa::Component;
use serde::{Serialize, Deserialize};
use r2d2::PooledConnection;
use diesel::{r2d2::ConnectionManager, PgConnection};

use crate::schema::{
  namespaces,
  git_repositories,
};

pub type Docker = bollard::Docker;
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DBConn = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Component, Serialize)]
pub struct PgDeleteGeneric {
  pub(crate) count: usize,
}

#[derive(Debug, Component, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="namespaces"]
pub struct NamespaceItem {
  pub(crate) id: Uuid,
  pub(crate) name: String,
}

#[derive(Component, Deserialize)]
pub struct NamespaceCreate {
  pub(crate) name: String,
}

#[derive(Debug, Component, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="git_repositories"]
pub struct GitRepositoryItem {
  pub(crate) id: Uuid,
  pub(crate) name: String,
  pub(crate) namespace: String,
  pub(crate) url: String,
  pub(crate) token: String,
}

#[derive(Component, Deserialize)]
pub struct GitRepositoryCreate {
  pub(crate) name: String,
  pub(crate) url: String,
  pub(crate) token: Option<String>,
}
