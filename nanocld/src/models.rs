use chrono::prelude::*;
use r2d2::PooledConnection;
use diesel_derive_enum::DbEnum;
use diesel::{r2d2::ConnectionManager, PgConnection};

use uuid::Uuid;
use serde::{Deserialize, Serialize, Deserializer};

use crate::{
  schema::{
    clusters, namespaces, git_repositories, cluster_networks,
    git_repository_branches, cargoes, cargo_proxy_configs, nginx_templates,
    cluster_variables, cluster_cargoes, cargo_environnements, nginx_logs,
  },
};

pub type DBConn = PooledConnection<ConnectionManager<PgConnection>>;
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[cfg(feature = "openapi")]
use utoipa::Component;

/// Generic postgresql delete response
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(Component))]
pub struct PgDeleteGeneric {
  pub(crate) count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(Component))]
pub struct PgGenericCount {
  pub(crate) count: i64,
}

/// Namespace to encapsulate clusters
/// this structure ensure read and write in database
#[derive(
  Debug,
  Serialize,
  Deserialize,
  Identifiable,
  Insertable,
  Queryable,
  Associations,
)]
#[primary_key(name)]
#[table_name = "namespaces"]
#[cfg_attr(feature = "openapi", derive(Component))]
pub struct NamespaceItem {
  pub(crate) name: String,
}

/// Partial namespace
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(Component))]
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
#[derive(Serialize, Deserialize, Debug, PartialEq, DbEnum, Clone)]
#[serde(rename_all = "snake_case")]
#[DieselType = "Git_repository_source_type"]
#[cfg_attr(feature = "openapi", derive(Component))]
pub enum GitRepositorySourceType {
  Github,
  Gitlab,
  Local,
}

/// Git repository are used to have project definition to deploy cargo
/// this structure ensure read and write entity in database
/// we also support git hooks such as create/delete branch
#[derive(
  Clone, Serialize, Deserialize, Insertable, Queryable, Identifiable,
)]
#[primary_key(name)]
#[table_name = "git_repositories"]
#[cfg_attr(feature = "openapi", derive(Component))]
pub struct GitRepositoryItem {
  pub(crate) name: String,
  pub(crate) url: String,
  pub(crate) default_branch: String,
  pub(crate) source: GitRepositorySourceType,
}

/// Partial Git repository
/// this structure ensure write entity in database
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(Component))]
pub struct GitRepositoryPartial {
  pub(crate) url: String,
  pub(crate) name: String,
}

/// Git repository branch
/// this structure ensure read and write entity in database
#[derive(
  Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Insertable,
)]
#[primary_key(key)]
#[table_name = "git_repository_branches"]
#[cfg_attr(feature = "openapi", derive(Component))]
pub struct GitRepositoryBranchItem {
  pub(crate) key: String,
  pub(crate) name: String,
  pub(crate) last_commit_sha: String,
  pub(crate) repository_name: String,
}

/// Partial git repository branch
/// this structure ensure write in database
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(Component))]
pub struct GitRepositoryBranchPartial {
  pub(crate) name: String,
  pub(crate) last_commit_sha: String,
  pub(crate) repository_name: String,
}

/// Partial cluster
/// this structure ensure write in database
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(Component))]
pub struct ClusterPartial {
  pub(crate) name: String,
}

/// Cluster used to encapsulate networks
/// this structure ensure read and write in database
#[derive(
  Debug,
  Clone,
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
#[cfg_attr(feature = "openapi", derive(Component))]
pub struct ClusterItem {
  pub(crate) key: String,
  pub(crate) name: String,
  pub(crate) namespace: String,
}

/// Cluster item with his relations
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(Component))]
pub struct ClusterItemWithRelation {
  pub(crate) key: String,
  pub(crate) name: String,
  pub(crate) namespace: String,
  pub(crate) networks: Option<Vec<ClusterNetworkItem>>,
}

/// Cluster network partial
/// this structure ensure write in database
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(Component))]
pub struct ClusterNetworkPartial {
  pub(crate) name: String,
}

/// Cluster network item
/// this structure ensure read and write in database
#[derive(
  Debug,
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
#[cfg_attr(feature = "openapi", derive(Component))]
pub struct ClusterNetworkItem {
  pub(crate) key: String,
  pub(crate) name: String,
  pub(crate) namespace: String,
  pub(crate) docker_network_id: String,
  pub(crate) cluster_key: String,
}

/// Cargo partial
/// this structure ensure write in database
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(Component))]
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
#[cfg_attr(feature = "openapi", derive(Component))]
pub struct CargoItem {
  pub(crate) key: String,
  pub(crate) namespace_name: String,
  pub(crate) name: String,
  pub(crate) image_name: String,
  pub(crate) binds: Vec<String>,
}

#[derive(
  Debug,
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
#[cfg_attr(feature = "openapi", derive(Component))]
pub struct CargoProxyConfigItem {
  pub(crate) cargo_key: String,
  pub(crate) domain_name: String,
  pub(crate) template: String,
  pub(crate) host_ip: String,
  pub(crate) target_port: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(Component))]
pub struct CargoProxyConfigPartial {
  pub(crate) domain_name: String,
  pub(crate) template: String,
  pub(crate) host_ip: String,
  pub(crate) target_port: i32,
}

#[derive(
  Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Insertable,
)]
#[primary_key(name)]
#[table_name = "nginx_templates"]
#[cfg_attr(feature = "openapi", derive(Component))]
pub struct NginxTemplateItem {
  pub(crate) name: String,
  pub(crate) content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(Component))]
pub struct ClusterJoinBody {
  pub(crate) cargo: String,
  pub(crate) network: String,
}

#[derive(
  Debug,
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
#[cfg_attr(feature = "openapi", derive(Component))]
pub struct ClusterVariableItem {
  pub(crate) key: String,
  pub(crate) cluster_key: String,
  pub(crate) name: String,
  pub(crate) value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(Component))]
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
#[cfg_attr(feature = "openapi", derive(Component))]
pub struct ClusterCargoItem {
  pub(crate) key: String,
  pub(crate) cargo_key: String,
  pub(crate) cluster_key: String,
  pub(crate) network_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(Component))]
pub struct ClusterCargoPartial {
  pub(crate) cargo_key: String,
  pub(crate) cluster_key: String,
  pub(crate) network_key: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(Component))]
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
#[cfg_attr(feature = "openapi", derive(Component))]
pub struct CargoEnvItem {
  pub(crate) key: String,
  pub(crate) cargo_key: String,
  pub(crate) name: String,
  pub(crate) value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(Component))]
pub struct ContainerImagePartial {
  pub(crate) name: String,
}

fn deserialize_empty_string<'de, D>(
  deserializer: D,
) -> Result<Option<String>, D::Error>
where
  D: Deserializer<'de>,
{
  let buf = String::deserialize(deserializer)?;
  if buf.is_empty() {
    Ok(None)
  } else {
    Ok(Some(buf))
  }
}

fn deserialize_string_to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
  D: Deserializer<'de>,
{
  let buf = String::deserialize(deserializer)?;
  let res = buf.parse::<i64>().unwrap_or_default();
  Ok(res)
}

fn deserialize_string_to_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
  D: Deserializer<'de>,
{
  let buf = String::deserialize(deserializer)?;
  let res = buf.parse::<i32>().unwrap_or_default();
  Ok(res)
}

fn deserialize_string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
  D: Deserializer<'de>,
{
  let buf = String::deserialize(deserializer)?;
  let res = buf.parse::<f64>().unwrap_or_default();
  Ok(res)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NginxLogPartial {
  pub(crate) date_gmt: DateTime<FixedOffset>,
  pub(crate) uri: String,
  pub(crate) host: String,
  pub(crate) remote_addr: String,
  pub(crate) realip_remote_addr: String,
  pub(crate) server_protocol: String,
  pub(crate) request_method: String,
  #[serde(deserialize_with = "deserialize_string_to_i64")]
  pub(crate) content_length: i64,
  #[serde(deserialize_with = "deserialize_string_to_i32")]
  pub(crate) status: i32,
  #[serde(deserialize_with = "deserialize_string_to_f64")]
  pub(crate) request_time: f64,
  #[serde(deserialize_with = "deserialize_string_to_i64")]
  pub(crate) body_bytes_sent: i64,
  #[serde(deserialize_with = "deserialize_empty_string")]
  pub(crate) proxy_host: Option<String>,
  #[serde(deserialize_with = "deserialize_empty_string")]
  pub(crate) upstream_addr: Option<String>,
  #[serde(deserialize_with = "deserialize_empty_string")]
  pub(crate) query_string: Option<String>,
  #[serde(deserialize_with = "deserialize_empty_string")]
  pub(crate) request_body: Option<String>,
  #[serde(deserialize_with = "deserialize_empty_string")]
  pub(crate) content_type: Option<String>,
  #[serde(deserialize_with = "deserialize_empty_string")]
  pub(crate) http_user_agent: Option<String>,
  #[serde(deserialize_with = "deserialize_empty_string")]
  pub(crate) http_referrer: Option<String>,
  #[serde(deserialize_with = "deserialize_empty_string")]
  pub(crate) http_accept_language: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "nginx_logs"]
pub struct NginxLogItem {
  pub(crate) key: Uuid,
  pub(crate) date_gmt: DateTime<FixedOffset>,
  pub(crate) uri: String,
  pub(crate) host: String,
  pub(crate) remote_addr: String,
  pub(crate) realip_remote_addr: String,
  pub(crate) server_protocol: String,
  pub(crate) request_method: String,
  pub(crate) content_length: i64,
  pub(crate) status: i32,
  pub(crate) request_time: f64,
  pub(crate) body_bytes_sent: i64,
  pub(crate) proxy_host: Option<String>,
  pub(crate) upstream_addr: Option<String>,
  pub(crate) query_string: Option<String>,
  pub(crate) request_body: Option<String>,
  pub(crate) content_type: Option<String>,
  pub(crate) http_user_agent: Option<String>,
  pub(crate) http_referrer: Option<String>,
  pub(crate) http_accept_language: Option<String>,
}

impl From<NginxLogPartial> for NginxLogItem {
  fn from(partial: NginxLogPartial) -> Self {
    NginxLogItem {
      key: Uuid::new_v4(),
      date_gmt: partial.date_gmt,
      uri: partial.uri,
      host: partial.host,
      remote_addr: partial.remote_addr,
      realip_remote_addr: partial.realip_remote_addr,
      server_protocol: partial.server_protocol,
      request_method: partial.request_method,
      status: partial.status,
      request_time: partial.request_time,
      content_length: partial.content_length,
      body_bytes_sent: partial.body_bytes_sent,
      proxy_host: partial.proxy_host,
      upstream_addr: partial.upstream_addr,
      query_string: partial.query_string,
      request_body: partial.request_body,
      content_type: partial.content_type,
      http_user_agent: partial.http_user_agent,
      http_referrer: partial.http_referrer,
      http_accept_language: partial.http_accept_language,
    }
  }
}

/// Re exports ours enums and diesel sql_types for schema.rs
pub mod exports {
  pub use diesel::sql_types::*;
  pub use super::Git_repository_source_type;
}
