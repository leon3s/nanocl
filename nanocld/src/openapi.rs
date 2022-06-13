use ntex::web;
use ntex_files as fs;
use utoipa::OpenApi;

use crate::controllers::*;
use crate::docker::models::{Ipam, IpamConfig, Network, NetworkContainer};
use crate::models::*;

#[derive(OpenApi)]
#[openapi(
  handlers(
    namespace::list,
    namespace::create,
    namespace::get_by_id_or_name,
    namespace::delete_by_id_or_name,
    cluster::list,
    cluster::create,
    git_repository::list,
    git_repository::create,
    git_repository::delete_by_id_or_name,
    cluster_network::list_networks,
  ),
  components(
    PgDeleteGeneric,
    NamespaceItem,
    NamespaceCreate,
    ClusterItem,
    Network,
    Ipam,
    IpamConfig,
    NetworkContainer,
    ClusterCreate,
    GitRepositoryItem,
    GitRepositoryCreate,
    GitRepositorySourceType,
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
