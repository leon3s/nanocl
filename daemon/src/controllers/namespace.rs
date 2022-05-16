use ntex::web;
use serde::{Serialize, Deserialize};

use crate::app_state::DaemonState;
use crate::datasources::mongo::models;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestError {
  status_code: u16,
  message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateResponse {
  id: String,
}

#[web::get("/namespaces")]
async fn get_namespace(req: web::HttpRequest) -> Result<web::HttpResponse, web::Error> {
    let state = match req.app_state::<DaemonState>() {
      Some(state)=> state,
      None => {
        return Ok(
          web::HttpResponse::InternalServerError()
          .content_type("application/json")
          .json(&RequestError {
            status_code: 500,
            message: "daemon error unable to get app_state".to_string(),
          })
        )
      },
    };
    let response = match state.repositories.namespace.list().await {
      Ok(response) => response,
      Err(err) => {
        return Ok(
          web::HttpResponse::InternalServerError()
          .content_type("application/json")
          .json(&RequestError {
            status_code: 500,
            message: format!("mongodb error {}", err),
          })
        );
      },
    };
    Ok(
        web::HttpResponse::Ok()
        .content_type("application/json")
        .json(&response)
    )
}

#[web::post("/namespaces")]
async fn post_namespace(req: web::HttpRequest, payload: web::types::Json<models::Namespace>) -> Result<web::HttpResponse, web::Error> {
  let new_namespace = payload.into_inner();
  let state = match req.app_state::<DaemonState>() {
    Some(state)=> state,
    None => {
      return Ok(
        web::HttpResponse::InternalServerError()
        .content_type("application/json")
        .json(&RequestError {
          status_code: 500,
          message: "daemon error unable to get app_state".to_string(),
        })
      )
    },
  };
  let id = match state.repositories.namespace.create(new_namespace).await {
    Ok(id) => id,
    Err(err) => {
      return Ok(
        web::HttpResponse::InternalServerError()
        .content_type("application/json")
        .json(&RequestError {
          status_code: 500,
          message: format!("mongodb error {}", err),
        })
      )
    }
  };
  Ok(
    web::HttpResponse::Ok()
    .content_type("application/json")
    .json(&CreateResponse {
      id,
    })
  )
}

pub fn ctrl_config(config: &mut web::ServiceConfig) {
  config.service(get_namespace);
  config.service(post_namespace);
}

#[cfg(test)]
mod ctrl_namespace_tests {
  use ntex::http::StatusCode;
  use ntex::web::{test, App, Error};

  use crate::app_state;
  use crate::controllers::namespace::*;
  use crate::datasources::mongo::models::Namespace;

  #[ntex::test]
  async fn test_get_namespace() -> Result<(), Error> {
    let state = app_state::init_state().await.unwrap();
    let srv = test::server(move || {
        App::new()
        .app_state(state.clone())
        .configure(ctrl_config)
    });

    let mut response = srv
    .get("/namespaces")
    .send()
    .await
    .unwrap();

    let status = response.status();
    response.json::<Vec<Namespace>>().await.unwrap();
    assert_eq!(status, StatusCode::OK);
    Ok(())
  }

  #[ntex::test]
  async fn test_post_namespace() -> Result<(), Error> {
    let state = app_state::init_state().await.unwrap();
    let srv = test::server(move || {
        App::new()
        .app_state(state.clone())
        .configure(ctrl_config)
    });
    let mut response = srv
    .post("/namespaces")
    .send_json(&models::Namespace {
      name: "test".to_string(),
      ..models::Namespace::default()
    })
    .await
    .unwrap();

    let status = response.status();
    response.json::<CreateResponse>().await.unwrap();
    assert_eq!(status, StatusCode::OK);
    Ok(())
  }
}
