use ntex::web;
use ntex_files as fs;
use utoipa::OpenApi;

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

    // Git repository
    git_repository::list_git_repository,
    git_repository::create_git_repository,
    git_repository::delete_git_repository_by_name,

    // Cluster
    cluster::list_cluster,
    cluster::create_cluster,
    cluster::delete_cluster_by_name,
    cluster::inspect_cluster_by_name,

    // Cluster network
    cluster_network::list_cluster_network,
    cluster_network::create_cluster_network,
    cluster_network::delete_cluster_network_by_name,
    cluster_network::inspect_cluster_network_by_name,
  ),
  components(
    ApiError,
    PgDeleteGeneric,

    // Git repository
    GitRepositoryItem,
    GitRepositoryPartial,
    GitRepositorySourceType,

    // Namespace
    NamespaceItem,
    NamespacePartial,

    // Cluster
    ClusterItem,
    ClusterPartial,

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
  config.service(
    fs::Files::new("/explorer", "./static/swagger").index_file("index.html"),
  );
}

#[cfg(test)]
mod test_openapi {
  use crate::utils::test::*;

  use super::*;

  async fn test_swagger(srv: &TestServer) -> TestReturn {
    let res = srv.get("/explorer").send().await?;
    assert!(res.status().is_success());
    Ok(())
  }

  async fn test_specs(srv: &TestServer) -> TestReturn {
    let res = srv.get("/explorer/swagger.json").send().await?;
    assert!(res.status().is_success());
    let content_type = match res.header("content-type") {
      None => "empty",
      Some(content_type) => content_type.to_str().unwrap(),
    };
    assert_eq!(content_type, "application/json");
    Ok(())
  }

  #[ntex::test]
  async fn main() -> TestReturn {
    let srv = generate_server(ntex_config);

    test_swagger(&srv).await?;
    test_specs(&srv).await?;
    Ok(())
  }
}
