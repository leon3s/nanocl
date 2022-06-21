use std::collections::HashMap;

use ntex::web;
use ntex::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::services::docker::build_image;
use crate::repositories::{cargo, namespace, cargo_ports};
use crate::models::{Pool, CargoPartial, CargoPortPartial};
use crate::utils::get_free_port;

use super::errors::HttpError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CargoQuery {
  pub(crate) namespace: Option<String>,
}

/// List cargo
#[utoipa::path(
  get,
  path = "/cargos",
  params(
    ("namespace" = Option<String>, query, description = "Name of the namespace where the cargo are stored"),
  ),
  responses(
    (status = 200, description = "List of cargo", body = [CargoItem]),
    (status = 400, description = "Generic database error", body = ApiError),
    (status = 404, description = "Namespace name not valid", body = ApiError),
  ),
)]
#[web::get("/cargos")]
pub async fn list_cargo(
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
  path = "/cargos",
  params(
    ("namespace" = Option<String>, query, description = "Name of the namespace where the cargo will be stored"),
  ),
  responses(
    (status = 201, description = "New cargo", body = CargoItem),
    (status = 400, description = "Generic database error", body = ApiError),
    (status = 404, description = "Namespace name not valid", body = ApiError),
  ),
)]
#[web::post("/cargos")]
pub async fn create_cargo(
  pool: web::types::State<Pool>,
  web::types::Query(qs): web::types::Query<CargoQuery>,
  web::types::Json(payload): web::types::Json<CargoPartial>,
) -> Result<web::HttpResponse, HttpError> {
  let nsp = match qs.namespace {
    None => String::from("global"),
    Some(nsp) => nsp,
  };
  let ports = payload.ports.clone();
  let item = cargo::create(nsp, payload, &pool).await?;
  if let Some(ports) = ports {
    let ports = ports
      .into_iter()
      .map(|port| CargoPortPartial {
        from: 0,
        to: port.parse::<i32>().unwrap_or(0),
      })
      .collect::<Vec<CargoPortPartial>>();
    cargo_ports::create_many_for_cargo(item.key.to_owned(), ports, &pool)
      .await?;
  }
  Ok(web::HttpResponse::Created().json(&item))
}

/// Delete cargo by it's name
#[utoipa::path(
  delete,
  path = "/cargos/{name}",
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
#[web::delete("/cargos/{name}")]
pub async fn delete_cargo_by_name(
  pool: web::types::State<Pool>,
  docker_api: web::types::State<bollard::Docker>,
  name: web::types::Path<String>,
  web::types::Query(qs): web::types::Query<CargoQuery>,
) -> Result<web::HttpResponse, HttpError> {
  log::debug!("requiring cargo deletion");
  let nsp = match qs.namespace {
    None => String::from("global"),
    Some(nsp) => nsp,
  };

  let gen_key = nsp + "-" + &name.into_inner();
  let item = cargo::find_by_key(gen_key.clone(), &pool).await?;
  let container_name =
    gen_key.to_owned() + "-" + &item.image_name.replace(':', "-");

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

  let res = cargo::delete_by_key(gen_key.clone(), &pool).await?;
  Ok(web::HttpResponse::Ok().json(&res))
}

/// Start cargo by it's name
#[utoipa::path(
  post,
  path = "/cargos/{name}/start",
  params(
    ("name" = String, path, description = "Name of cargo to start"),
    ("namespace" = Option<String>, query, description = "Name of the namespace where the cargo is stored"),
  ),
  responses(
    (status = 204, description = "Cargo started"),
    (status = 400, description = "Generic database error", body = ApiError),
    (status = 404, description = "Namespace name not valid", body = ApiError),
  ),
)]
#[web::post("/cargos/{name}/start")]
pub async fn start_cargo_by_name(
  pool: web::types::State<Pool>,
  docker_api: web::types::State<bollard::Docker>,
  name: web::types::Path<String>,
  web::types::Query(qs): web::types::Query<CargoQuery>,
) -> Result<web::HttpResponse, HttpError> {
  let nsp = match qs.namespace {
    None => String::from("global"),
    Some(nsp) => nsp,
  };

  let gen_key = nsp + "-" + &name.into_inner();
  let item = cargo::find_by_key(gen_key.clone(), &pool).await?;
  let image_name = item.image_name.clone();
  let container_name = gen_key.to_owned() + "-" + &image_name.replace(':', "-");

  let ports = cargo_ports::list_for_cargo(item.to_owned(), &pool).await?;

  log::debug!("item found {:?}", item);
  log::debug!("image name not empty {:?}", image_name.clone());
  if docker_api.inspect_image(&item.image_name).await.is_err() {
    return Err(HttpError {
      msg: String::from("you need to build cargo before run it."),
      status: StatusCode::BAD_REQUEST,
    });
  }
  let image = Some(image_name.clone());
  let options = Some(bollard::container::CreateContainerOptions {
    name: container_name.clone(),
  });
  let mut port_bindings: HashMap<
    String,
    Option<Vec<bollard::models::PortBinding>>,
  > = HashMap::new();
  ports.into_iter().for_each(|port| {
    let new_port = get_free_port().unwrap();
    port_bindings.insert(
      port.to.to_string() + "/tcp",
      Some(vec![bollard::models::PortBinding {
        host_ip: None,
        host_port: Some(new_port.to_string()),
      }]),
    );
  });
  let config = bollard::container::Config {
    image,
    tty: Some(true),
    host_config: Some(bollard::models::HostConfig {
      port_bindings: Some(port_bindings),
      ..Default::default()
    }),
    attach_stdout: Some(true),
    attach_stderr: Some(true),
    ..Default::default()
  };
  if let Err(err) = docker_api.create_container(options, config).await {
    return Err(HttpError {
      msg: format!("unable to create container {:?}", err),
      status: StatusCode::BAD_REQUEST,
    });
  }

  if let Err(err) = docker_api
    .start_container(
      &container_name,
      None::<bollard::container::StartContainerOptions<String>>,
    )
    .await
  {
    return Err(HttpError {
      msg: format!("unable to start container {:?}", err),
      status: StatusCode::BAD_REQUEST,
    });
  }
  Ok(web::HttpResponse::NoContent().into())
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(list_cargo);
  config.service(create_cargo);
  // config.service(build_cargo_by_name);
  config.service(delete_cargo_by_name);
  config.service(start_cargo_by_name);
}

#[cfg(test)]
mod test_cargo {
  use crate::utils::test::*;

  use crate::models::CargoPartial;

  use super::ntex_config;

  #[ntex::test]
  async fn test_list() -> TestReturn {
    let srv = generate_server(ntex_config);
    let mut res = srv.get("/cargos").send().await?;
    println!("body {:?}", res.body().await);
    assert!(res.status().is_success());
    Ok(())
  }

  #[ntex::test]
  async fn test_start_nginx() -> TestReturn {
    const CARGO_NAME: &str = "nginx-test";
    let srv = generate_server(ntex_config);

    let res = srv
      .post("/cargos")
      .send_json(&CargoPartial {
        name: String::from(CARGO_NAME),
        network_name: None,
        image_name: String::from("nginx:latest"),
      })
      .await?;
    assert!(res.status().is_success());

    let res = srv
      .post(format!("/cargos/{name}/start", name = CARGO_NAME))
      .send()
      .await?;
    assert!(res.status().is_success());

    let res = srv
      .delete(format!("/cargos/{name}", name = CARGO_NAME))
      .send()
      .await?;
    assert!(res.status().is_success());
    Ok(())
  }
}
