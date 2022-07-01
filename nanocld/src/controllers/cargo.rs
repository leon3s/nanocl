use ntex::web;
use ntex::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::repositories::{cargo, namespace, cargo_proxy_config};
use crate::models::{Pool, CargoPartial};

use super::errors::HttpError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CargoQuery {
  pub(crate) namespace: Option<String>,
}

/// List cargo
#[utoipa::path(
  get,
  path = "/cargoes",
  params(
    ("namespace" = Option<String>, query, description = "Name of the namespace where the cargo are stored"),
  ),
  responses(
    (status = 200, description = "List of cargo", body = [CargoItem]),
    (status = 400, description = "Generic database error", body = ApiError),
    (status = 404, description = "Namespace name not valid", body = ApiError),
  ),
)]
#[web::get("/cargoes")]
async fn list_cargo(
  pool: web::types::State<Pool>,
  web::types::Query(qs): web::types::Query<CargoQuery>,
) -> Result<web::HttpResponse, HttpError> {
  let nsp = match qs.namespace {
    None => String::from("global"),
    Some(nsp) => nsp,
  };

  let nsp = namespace::find_by_name(nsp, &pool).await?;
  let items = cargo::find_by_namespace(nsp, &pool).await?;
  Ok(web::HttpResponse::Ok().json(&items))
}

/// Create new cargo
#[utoipa::path(
  post,
  request_body = CargoPartial,
  path = "/cargoes",
  params(
    ("namespace" = Option<String>, query, description = "Name of the namespace where the cargo will be stored"),
  ),
  responses(
    (status = 201, description = "New cargo", body = CargoItem),
    (status = 400, description = "Generic database error", body = ApiError),
    (status = 404, description = "Namespace name not valid", body = ApiError),
  ),
)]
#[web::post("/cargoes")]
async fn create_cargo(
  pool: web::types::State<Pool>,
  web::types::Query(qs): web::types::Query<CargoQuery>,
  web::types::Json(payload): web::types::Json<CargoPartial>,
) -> Result<web::HttpResponse, HttpError> {
  let nsp = match qs.namespace {
    None => String::from("global"),
    Some(nsp) => nsp,
  };
  log::info!(
    "creating cargo for namespace {} with payload {:?}",
    &nsp,
    payload,
  );
  let proxy_config = payload.proxy_config.clone();
  let item = cargo::create(nsp, payload, &pool).await?;
  if let Some(proxy_config) = proxy_config {
    log::info!("creating proxy config");
    cargo_proxy_config::create_for_cargo(
      item.key.to_owned(),
      proxy_config,
      &pool,
    )
    .await?;
  }
  log::info!("cargo succefully created");
  Ok(web::HttpResponse::Created().json(&item))
}

/// Delete cargo by it's name
#[utoipa::path(
  delete,
  path = "/cargoes/{name}",
  params(
    ("name" = String, path, description = "Name of the cargo"),
    ("namespace" = Option<String>, query, description = "Name of the namespace where the cargo is stored"),
  ),
  responses(
    (status = 200, description = "Generic delete", body = PgDeleteGeneric),
    (status = 400, description = "Generic database error", body = ApiError),
    (status = 404, description = "Namespace name not valid", body = ApiError),
  ),
)]
#[web::delete("/cargoes/{name}")]
async fn delete_cargo_by_name(
  pool: web::types::State<Pool>,
  docker_api: web::types::State<bollard::Docker>,
  name: web::types::Path<String>,
  web::types::Query(qs): web::types::Query<CargoQuery>,
) -> Result<web::HttpResponse, HttpError> {
  log::info!("asking cargo deletion {}", &name);
  let nsp = match qs.namespace {
    None => String::from("global"),
    Some(nsp) => nsp,
  };

  let gen_key = nsp + "-" + &name.into_inner();
  let item = cargo::find_by_key(gen_key.clone(), &pool).await?;
  let container_name =
    gen_key.to_owned() + "-" + &item.image_name.replace(':', "-");

  log::info!("deleting container {}", &container_name);
  let options = Some(bollard::container::RemoveContainerOptions {
    force: true,
    ..Default::default()
  });

  let res = docker_api.inspect_container(&container_name, None).await;

  if res.is_ok() {
    let res = docker_api.remove_container(&container_name, options).await;
    if let Err(err) = res {
      return Err(HttpError {
        msg: format!("unable to remove container {:?}", err),
        status: StatusCode::INTERNAL_SERVER_ERROR,
      });
    }
  }

  log::info!("deleting cargo proxy config");
  cargo_proxy_config::delete_for_cargo(gen_key.to_owned(), &pool).await?;
  log::info!("deleting ports cargo");
  let res = cargo::delete_by_key(gen_key.to_owned(), &pool).await?;
  Ok(web::HttpResponse::Ok().json(&res))
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(list_cargo);
  config.service(create_cargo);
  config.service(delete_cargo_by_name);
}

#[cfg(test)]
mod test_cargo {
  use crate::utils::test::*;

  use super::ntex_config;

  #[ntex::test]
  async fn test_list() -> TestReturn {
    let srv = generate_server(ntex_config);
    let mut res = srv.get("/cargoes").send().await?;
    println!("body {:#?}", res.body().await);
    assert!(res.status().is_success());
    Ok(())
  }
}
