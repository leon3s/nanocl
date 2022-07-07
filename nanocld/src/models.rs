use utoipa::Component;
use r2d2::PooledConnection;
use diesel_derive_enum::DbEnum;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};

use crate::schema::{
  clusters, namespaces, git_repositories, cluster_networks,
  git_repository_branches, cargoes, cargo_proxy_configs, nginx_templates,
  cluster_variables, cluster_cargoes, cargo_environnements,
};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DBConn = PooledConnection<ConnectionManager<PgConnection>>;

/// Generic postgresql delete response
#[derive(Debug, Component, Serialize, Deserialize)]
pub struct PgDeleteGeneric {
  pub(crate) count: usize,
}

#[derive(Debug, Component, Serialize, Deserialize)]
pub struct PgGenericCount {
  pub(crate) count: i64,
}

/// Namespace to encapsulate clusters
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
)]
#[primary_key(name)]
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
  Component, Clone, Serialize, Deserialize, Insertable, Queryable, Identifiable,
)]
#[primary_key(name)]
#[table_name = "git_repositories"]
pub struct GitRepositoryItem {
  pub(crate) name: String,
  pub(crate) url: String,
  pub(crate) default_branch: String,
  pub(crate) source: GitRepositorySourceType,
}

/// Partial Git repository
/// this structure ensure write entity in database
#[derive(Component, Serialize, Deserialize)]
pub struct GitRepositoryPartial {
  pub(crate) url: String,
  pub(crate) name: String,
}

/// Git repository branch
/// this structure ensure read and write entity in database
#[derive(
  Debug,
  Clone,
  Component,
  Serialize,
  Deserialize,
  Queryable,
  Identifiable,
  Insertable,
)]
#[primary_key(key)]
#[table_name = "git_repository_branches"]
pub struct GitRepositoryBranchItem {
  pub(crate) key: String,
  pub(crate) name: String,
  pub(crate) last_commit_sha: String,
  pub(crate) repository_name: String,
}

/// Partial git repository branch
/// this structure ensure write in database
#[derive(Component, Serialize, Deserialize)]
pub struct GitRepositoryBranchPartial {
  pub(crate) name: String,
  pub(crate) last_commit_sha: String,
  pub(crate) repository_name: String,
}

/// Partial cluster
/// this structure ensure write in database
#[derive(Component, Serialize, Deserialize)]
pub struct ClusterPartial {
  pub(crate) name: String,
}

/// Cluster used to encapsulate networks
/// this structure ensure read and write in database
#[derive(
  Debug,
  Clone,
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

/// Cluster item with his relations
#[derive(Component, Serialize, Deserialize)]
pub struct ClusterItemWithRelation {
  pub(crate) key: String,
  pub(crate) name: String,
  pub(crate) namespace: String,
  pub(crate) networks: Option<Vec<ClusterNetworkItem>>,
}

/// Cluster network partial
/// this structure ensure write in database
#[derive(Component, Serialize, Deserialize)]
pub struct ClusterNetworkPartial {
  pub(crate) name: String,
}

/// Cluster network item
/// this structure ensure read and write in database
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
  pub(crate) namespace: String,
  pub(crate) docker_network_id: String,
  pub(crate) cluster_key: String,
}

/// Cargo partial
/// this structure ensure write in database
#[derive(Debug, Component, Serialize, Deserialize)]
pub struct CargoPartial {
  pub(crate) name: String,
  pub(crate) image_name: String,
  pub(crate) environnements: Option<Vec<String>>,
  pub(crate) proxy_config: Option<CargoProxyConfigPartial>,
  pub(crate) binds: Option<Vec<String>>,
}

/// Cargo item is an definition to container create image and start them
/// this structure ensure read and write in database
#[derive(
  Debug,
  Clone,
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
#[belongs_to(NamespaceItem, foreign_key = "namespace_name")]
#[table_name = "cargoes"]
pub struct CargoItem {
  pub(crate) key: String,
  pub(crate) name: String,
  pub(crate) image_name: String,
  pub(crate) namespace_name: String,
  pub(crate) binds: Vec<String>,
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
#[primary_key(cargo_key)]
#[table_name = "cargo_proxy_configs"]
pub struct CargoProxyConfigItem {
  pub(crate) cargo_key: String,
  pub(crate) domain_name: String,
  pub(crate) template: String,
  pub(crate) host_ip: String,
  pub(crate) target_port: i32,
}

#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct CargoProxyConfigPartial {
  pub(crate) domain_name: String,
  pub(crate) template: String,
  pub(crate) host_ip: String,
  pub(crate) target_port: i32,
}

#[derive(
  Debug,
  Clone,
  Component,
  Serialize,
  Deserialize,
  Queryable,
  Identifiable,
  Insertable,
)]
#[primary_key(name)]
#[table_name = "nginx_templates"]
pub struct NginxTemplateItem {
  pub(crate) name: String,
  pub(crate) content: String,
}

#[derive(Debug, Component, Serialize, Deserialize)]
pub struct ClusterJoinBody {
  pub(crate) cargo: String,
  pub(crate) network: String,
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
#[table_name = "cluster_variables"]
#[belongs_to(ClusterItem, foreign_key = "cluster_key")]
pub struct ClusterVariableItem {
  pub(crate) key: String,
  pub(crate) cluster_key: String,
  pub(crate) name: String,
  pub(crate) value: String,
}

#[derive(Debug, Component, Serialize, Deserialize)]
pub struct ClusterVariablePartial {
  pub(crate) name: String,
  pub(crate) value: String,
}

#[derive(
  Debug,
  Serialize,
  Deserialize,
  Queryable,
  Insertable,
  Identifiable,
  Associations,
  AsChangeset,
)]
#[primary_key(key)]
#[table_name = "cluster_cargoes"]
#[belongs_to(CargoItem, foreign_key = "cargo_key")]
#[belongs_to(ClusterItem, foreign_key = "cluster_key")]
#[belongs_to(ClusterNetworkItem, foreign_key = "network_key")]
pub struct ClusterCargoItem {
  pub(crate) key: String,
  pub(crate) cargo_key: String,
  pub(crate) cluster_key: String,
  pub(crate) network_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterCargoPartial {
  pub(crate) cargo_key: String,
  pub(crate) cluster_key: String,
  pub(crate) network_key: String,
}

#[derive(Debug, Clone)]
pub struct CargoEnvPartial {
  pub(crate) cargo_key: String,
  pub(crate) name: String,
  pub(crate) value: String,
}

#[derive(
  Debug,
  Serialize,
  Deserialize,
  Queryable,
  Insertable,
  Identifiable,
  Associations,
  AsChangeset,
)]
#[primary_key(key)]
#[table_name = "cargo_environnements"]
pub struct CargoEnvItem {
  pub(crate) key: String,
  pub(crate) cargo_key: String,
  pub(crate) name: String,
  pub(crate) value: String,
}

/// Re exports ours enums and diesel sql_types for schema.rs
pub mod exports {
  pub use diesel::sql_types::*;
  pub use super::Git_repository_source_type;
}
