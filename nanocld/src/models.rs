use uuid::Uuid;
use utoipa::Component;
use r2d2::PooledConnection;
use diesel_derive_enum::DbEnum;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};

use crate::schema::{
  clusters, namespaces, git_repositories, cluster_networks,
  git_repository_branches,
};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DBConn = PooledConnection<ConnectionManager<PgConnection>>;

/// Generic postgresql delete response
#[derive(Component, Serialize, Deserialize)]
pub struct PgDeleteGeneric {
  pub(crate) count: usize,
}

/// Namespace to encapsulate clusters
/// this structure ensure read and write in database
#[derive(Debug, Component, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "namespaces"]
pub struct NamespaceItem {
  pub(crate) name: String,
}

/// Partial namespace
#[derive(Component, Serialize, Deserialize)]
pub struct NamespacePartial {
  pub(crate) name: String,
}

/// Git repository source types
/// # Examples
/// ```
/// GitRepositorySourceType::Github; // For github.com
/// GitRepositorySourceType::Gitlab; // for gitlab.com
/// GitRepositorySourceType::Local; // for nanocl managed git repository
/// ```
#[derive(
  Component, Serialize, Deserialize, Debug, PartialEq, DbEnum, Clone,
)]
#[serde(rename_all = "snake_case")]
#[DieselType = "Git_repository_source_type"]
pub enum GitRepositorySourceType {
  Github,
  Gitlab,
  Local,
}

/// Git repository are used to have project definition to deploy cargo
/// this structure ensure read and write entity in database
/// we also support git hooks such as create/delete branch
#[derive(
  Component, Serialize, Deserialize, Insertable, Queryable, Identifiable,
)]
#[table_name = "git_repositories"]
pub struct GitRepositoryItem {
  pub(crate) id: Uuid,
  pub(crate) name: String,
  pub(crate) url: String,
  pub(crate) token: Option<String>,
  pub(crate) source: GitRepositorySourceType,
}

/// Partial Git repository
/// this structure ensure write entity in database
#[derive(Component, Serialize, Deserialize)]
pub struct GitRepositoryPartial {
  pub(crate) url: String,
  pub(crate) name: String,
  pub(crate) token: Option<String>,
}

/// Git repository branch
/// this structure ensure read and write entity in database
#[derive(Debug, Component, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "git_repository_branches"]
pub struct GitRepositoryBranchItem {
  pub(crate) id: Uuid,
  pub(crate) name: String,
  pub(crate) repository_id: Uuid,
}

/// Partial git repository branch
/// this structure ensure write in database
#[derive(Component, Serialize, Deserialize)]
pub struct GitRepositoryBranchPartial {
  pub(crate) name: String,
  pub(crate) repository_id: Uuid,
}

/// Cluster used to encapsulate networks
/// this structure ensure read and write in database
#[derive(
  Debug,
  Component,
  Serialize,
  Deserialize,
  Identifiable,
  Insertable,
  Queryable,
  Associations,
  AsChangeset,
)]
#[primary_key(key)]
#[table_name = "clusters"]
pub struct ClusterItem {
  pub(crate) key: String,
  pub(crate) name: String,
  pub(crate) namespace: String,
}

#[derive(Component, Serialize, Deserialize)]
pub struct ClusterItemWithRelation {
  pub(crate) key: String,
  pub(crate) name: String,
  pub(crate) namespace: String,
  pub(crate) networks: Option<Vec<ClusterNetworkItem>>,
}

/// Partial cluster
/// this structure ensure write in database
#[derive(Component, Serialize, Deserialize)]
pub struct ClusterPartial {
  pub(crate) name: String,
}

#[derive(Component, Serialize, Deserialize)]
pub struct ClusterNetworkPartial {
  pub(crate) name: String,
}

#[derive(
  Debug,
  Component,
  Serialize,
  Deserialize,
  Queryable,
  Identifiable,
  Insertable,
  Associations,
  AsChangeset,
)]
#[primary_key(key)]
#[belongs_to(ClusterItem, foreign_key = "cluster_key")]
#[table_name = "cluster_networks"]
pub struct ClusterNetworkItem {
  pub(crate) key: String,
  pub(crate) name: String,
  pub(crate) docker_network_id: String,
  pub(crate) cluster_key: String,
}

/// Rexports postgre enum for schema.rs
pub mod exports {
  pub use super::Git_repository_source_type;
}
