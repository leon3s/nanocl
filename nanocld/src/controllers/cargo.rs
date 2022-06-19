use ntex::web;
use futures::StreamExt;
use ntex::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::services::docker::{build_image, build_git_repository};
use crate::repositories::{cargo, namespace, git_repository};
use crate::models::{Pool, CargoPartial};

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
    None => String::from("default"),
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
    None => String::from("default"),
    Some(nsp) => nsp,
  };

  let item = cargo::create(nsp, payload, &pool).await?;
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
  docker: web::types::State<bollard::Docker>,
  name: web::types::Path<String>,
  web::types::Query(qs): web::types::Query<CargoQuery>,
) -> Result<web::HttpResponse, HttpError> {
  let nsp = match qs.namespace {
    None => String::from("default"),
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
  let res = docker.remove_container(&container_name, options).await;
  if let Err(err) = res {
    return Err(HttpError {
      msg: format!("unable to remove container {:?}", err),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    });
  }

  let res = cargo::delete_by_key(gen_key.clone(), &pool).await?;
  Ok(web::HttpResponse::Ok().json(&res))
}

/// Build a cargo by it's name
#[utoipa::path(
  post,
  path = "/cargos/{name}/build",
  params(
    ("name" = String, path, description = "Name of the cargo"),
    ("namespace" = Option<String>, query, description = "Name of the namespace where the cargo is stored"),
  ),
  responses(
    (status = 200, description = "Generic delete", body = String, content_type = "nanocl/streaming-v1"),
    (status = 400, description = "Generic database error", body = ApiError),
    (status = 404, description = "Namespace name not valid", body = ApiError),
  ),
)]
#[web::post("/cargos/{name}/build")]
pub async fn build_cargo_by_name(
  pool: web::types::State<Pool>,
  docker: web::types::State<bollard::Docker>,
  name: web::types::Path<String>,
  web::types::Query(qs): web::types::Query<CargoQuery>,
) -> Result<web::HttpResponse, HttpError> {
  let nsp = match qs.namespace {
    None => String::from("default"),
    Some(nsp) => nsp,
  };
  let gen_key = nsp + "-" + &name.into_inner();
  let item = cargo::find_by_key(gen_key, &pool).await?;

  if !item.image_name.is_empty() {
    let stream = build_image(item.image_name, docker).await?;
    return Ok(
      web::HttpResponse::Ok()
        .content_type("nanocl/streaming-v1")
        .streaming(stream),
    );
  }
  if item.repository_name.is_empty() {
    return Err(HttpError {
      msg: String::from("cargo datastructure error image_name and repository_name cannot be both empty."),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    });
  }
  let git =
    git_repository::find_by_id_or_name(item.repository_name, &pool).await?;
  let stream = build_git_repository(docker, git).await?;
  Ok(
    web::HttpResponse::Ok()
      .content_type("nanocl/streaming-v1")
      .streaming(stream),
  )
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
  docker: web::types::State<bollard::Docker>,
  name: web::types::Path<String>,
  web::types::Query(qs): web::types::Query<CargoQuery>,
) -> Result<web::HttpResponse, HttpError> {
  let nsp = match qs.namespace {
    None => String::from("default"),
    Some(nsp) => nsp,
  };

  let gen_key = nsp + "-" + &name.into_inner();
  let item = cargo::find_by_key(gen_key.clone(), &pool).await?;
  let image_name = item.image_name.clone();
  let container_name = gen_key.to_owned() + "-" + &image_name.replace(':', "-");

  println!("item found {:?}", item);
  if !&item.image_name.is_empty() {
    println!("image name not empty {:?}", image_name.clone());
    if docker.inspect_image(&item.image_name).await.is_err() {
      return Err(HttpError {
        msg: String::from("you need to build cargo before run it."),
        status: StatusCode::BAD_REQUEST,
      });
    }
    let image = Some(image_name.clone());
    let options = Some(bollard::container::CreateContainerOptions {
      name: container_name.clone(),
    });
    let config = bollard::container::Config {
      image,
      tty: Some(true),
      attach_stdout: Some(true),
      attach_stderr: Some(true),
      ..Default::default()
    };
    if let Err(err) = docker.create_container(options, config).await {
      return Err(HttpError {
        msg: format!("unable to create container {:?}", err),
        status: StatusCode::BAD_REQUEST,
      });
    }
  }

  if let Err(err) = docker
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
  config.service(build_cargo_by_name);
  config.service(delete_cargo_by_name);
  config.service(start_cargo_by_name);
}
