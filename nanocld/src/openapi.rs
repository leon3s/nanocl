use ntex::web;
use utoipa::OpenApi;

#[allow(unused_imports)]
use ntex_files as fs;

use crate::models::*;
use crate::controllers::*;
use crate::controllers::errors::ApiError;

#[derive(OpenApi)]
#[openapi(
  handlers(
    // Namespace
    namespace::list_namespace,
    namespace::create_namespace,
    namespace::delete_namespace_by_name,
    namespace::inspect_namespace_by_name,

    // nginx template
    nginx_template::list_nginx_template,

    // Cargo
    cargo::list_cargo,
    cargo::create_cargo,
    cargo::delete_cargo_by_name,
    cargo::count_cargo,

    // Git repository
    git_repository::list_git_repository,
    git_repository::create_git_repository,
    git_repository::build_git_repository_by_name,
    git_repository::delete_git_repository_by_name,

    // Cluster
    cluster::list_cluster,
    cluster::count_cluster,
    cluster::create_cluster,
    cluster::delete_cluster_by_name,
    cluster::inspect_cluster_by_name,
    cluster::start_cluster_by_name,
    cluster::join_cargo_to_cluster,

    // Cluster variable
    cluster_variable::list_cluster_variable,
    cluster_variable::create_cluster_variable,
    cluster_variable::delete_cluster_variable,

    // Cluster network
    cluster_network::list_cluster_network,
    cluster_network::create_cluster_network,
    cluster_network::delete_cluster_network_by_name,
    cluster_network::inspect_cluster_network_by_name,
    cluster_network::count_cluster_network_by_namespace,
  ),
  components(
    ApiError,
    PgDeleteGeneric,
    PgGenericCount,

    // Nginx template
    NginxTemplateItem,

    // Git repository
    GitRepositoryItem,
    GitRepositoryPartial,
    GitRepositorySourceType,

    // Namespace
    NamespaceItem,
    NamespacePartial,

    // Cargo
    CargoItem,
    CargoPartial,
    CargoProxyConfigItem,
    CargoProxyConfigPartial,

    // Cluster
    ClusterItem,
    ClusterPartial,
    ClusterJoinBody,

    // Cluster variable
    ClusterVariableItem,
    ClusterVariablePartial,

    // Cluster network
    ClusterNetworkItem,
    ClusterNetworkPartial,
    ClusterItemWithRelation,

    // Todo Docker network struct bindings
    // Network,
    // Ipam,
    // IpamConfig,
    // NetworkContainer,
  )
)]
struct ApiDoc;

#[web::get("/explorer/swagger.json")]
async fn get_api_specs() -> Result<web::HttpResponse, web::Error> {
  let api_spec = ApiDoc::openapi().to_pretty_json().unwrap();
  Ok(
    web::HttpResponse::Ok()
      .content_type("application/json")
      .body(&api_spec),
  )
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(get_api_specs);
  // config.service(
  //   fs::Files::new("/explorer", "./static/swagger").index_file("index.html"),
  // );
}
