use uuid::Uuid;
use utoipa::Component;
use r2d2::PooledConnection;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use diesel::{r2d2::ConnectionManager, PgConnection};
use crate::schema::{
    clusters,
    namespaces,
    git_repositories,
    git_repository_branches,
};

pub type Docker = bollard::Docker;
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DBConn = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Component, Serialize, Deserialize)]
pub struct PgDeleteGeneric {
    pub(crate) count: usize,
}

#[derive(Debug, Component, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "namespaces"]
pub struct NamespaceItem {
    pub(crate) id: Uuid,
    pub(crate) name: String,
}

#[derive(Component, Serialize, Deserialize)]
pub struct NamespaceCreate {
    pub(crate) name: String,
}

#[derive(Component, Serialize, Deserialize, Debug, PartialEq, DbEnum, Clone)]
#[serde(rename_all = "snake_case")]
#[DieselType = "Git_repository_source_type"]
pub enum GitRepositorySourceType {
    Github,
    Gitlab,
    Local,
}

#[derive(Component, Serialize, Deserialize, Insertable, Queryable, Identifiable)]
#[table_name = "git_repositories"]
pub struct GitRepositoryItem {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) token: Option<String>,
    pub(crate) source: GitRepositorySourceType,
}

#[derive(Debug, Component, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "git_repository_branches"]
pub struct GitRepositoryBranchItem {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) repository_id: Uuid,
}

#[derive(Component, Serialize, Deserialize)]
pub struct GitRepositoryBranchCreate {
    pub(crate) name: String,
    pub(crate) repository_id: Uuid,
}

#[derive(Component, Serialize, Deserialize)]
pub struct GitRepositoryCreate {
    pub(crate) url: String,
    pub(crate) name: String,
    pub(crate) token: Option<String>,
}

#[derive(Component, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "clusters"]
pub struct ClusterItem {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) gen_id: String,
    pub(crate) namespace: String,
}

#[derive(Component, Serialize, Deserialize)]
pub struct ClusterCreate {
    pub(crate) name: String,
}

pub mod exports {
    pub use super::Git_repository_source_type;
}
