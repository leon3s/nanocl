use mongodb::bson::doc;
use ntex::web;

use crate::app_state::DaemonState;
use crate::datasources::mongo::models;
use crate::responses::errors;
use crate::responses::models::{CreateResponse, DeleteResponse};

#[web::get("/namespaces")]
async fn get_namespace(
  state: web::types::State<DaemonState>,
) -> Result<web::HttpResponse, errors::HttpError> {
    let namespace = &state.repositories.namespace;
    let response = match namespace.find().await {
      Ok(response) => response,
      Err(err) => {
        return Err(
          errors::mongo_error(err)
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
async fn post_namespace(
  state: web::types::State<DaemonState>,
  payload: web::types::Json<models::Namespace>
) -> Result<web::HttpResponse, errors::HttpError> {
  let namespace = &state.repositories.namespace;
  let id = match namespace.create(payload.into_inner()).await {
    Ok(success_resp) => success_resp,
    Err(err) => {
      return Err(errors::mongo_error(err));
    },
  };
  Ok(
    web::HttpResponse::Created()
    .content_type("application/json")
    .json(&CreateResponse {
      id,
    })
  )
}

#[derive(Debug, serde::Deserialize)]
struct DeleteQuery {
  m_where: models::Namespace,
}

#[web::delete("/namespaces")]
async fn delete_namespace(
  state: web::types::State<DaemonState>,
  web::types::Query(query): web::types::Query<DeleteQuery>
) -> Result<web::HttpResponse, errors::HttpError> {
  let namespace = &state.repositories.namespace;
  println!("query : {:?}", query.m_where);
  let count = match namespace.delete(doc! {
    "_id": query.m_where.id,
    "name": query.m_where.name,
  }).await {
    Ok(count) => count,
    Err(err) => {
      return Err(errors::mongo_error(err));
    }
  };
  Ok(
    web::HttpResponse::Accepted()
    .content_type("application/json")
    .json(&DeleteResponse {
      count
    })
  )
}

#[web::get("/namespaces/{id}")]
async fn delete_namespace_by_id(
  state: web::types::State<DaemonState>,
  id: web::types::Path<String>,
) -> Result<web::HttpResponse, errors::HttpError> {
  let namespace = &state.repositories.namespace;
  let count = match namespace.delete_by_id(id.to_owned()).await {
    Ok(count) => count,
    Err(err) => {
      return Err(errors::mongo_error(err));
    }
  };
  Ok(
    web::HttpResponse::Accepted()
    .content_type("application/json")
    .json(&DeleteResponse {
      count
    })
  )
}

pub fn ctrl_config(config: &mut web::ServiceConfig) {
  config.service(get_namespace);
  config.service(post_namespace);
  config.service(delete_namespace_by_id);
}

#[cfg(test)]
mod ctrl_namespace_tests {
  use ntex::http::StatusCode;
  use ntex::web::{test, App, Error};

  use crate::{app_state, responses};
  use crate::controllers::namespace::*;
  use crate::datasources::mongo::models;

  #[ntex::test]
  async fn test_post_namespace() -> Result<(), Error> {
    // Generate daemon state
    let state = app_state::init_state().await.unwrap();
    let namespace = state.repositories.namespace.clone();
    // Generate server
    let srv = test::server(move || {
        App::new()
        .state(state.clone())
        .configure(ctrl_config)
    });

    // Test post request
    let mut resp = srv
    .post("/namespaces")
    .send_json(&models::Namespace {
      name: "test".to_string(),
      ..models::Namespace::default()
    })
    .await
    .unwrap();
    let resp_status = resp.status();
    let resp_body = resp.json::<CreateResponse>().await.unwrap();
    assert_eq!(resp_status, StatusCode::CREATED);

    // Clean entry
    let count = namespace.delete_by_id(resp_body.id).await.unwrap();
    assert_eq!(count, 1);
    Ok(())
  }

  #[ntex::test]
  async fn test_get_namespace() -> Result<(), Error> {
    // Generate daemon state
    let state = app_state::init_state().await.unwrap();
    let namespace = state.repositories.namespace.clone();
    // Generate server
    let srv = test::server(move || {
        App::new()
        .state(state.clone())
        .configure(ctrl_config)
    });

    // Test get request
    let mut resp = srv
    .get("/namespaces")
    .send()
    .await
    .unwrap();
    let resp_status = resp.status();
    let resp_body = resp.json::<Vec<models::Namespace>>().await.unwrap();
    assert_eq!(resp_status, StatusCode::OK);
    let to_match = namespace.find().await.unwrap();
    assert_eq!(resp_body.len(), to_match.len());
    Ok(())
  }

  #[ntex::test]
  async fn test_delete_by_id() -> Result<(), Error> {
    let state = app_state::init_state().await.unwrap();
    let srv = test::server(move || {
        App::new()
        .state(state.clone())
        .configure(ctrl_config)
    });

    let test_namespace = models::Namespace {
      name: String::from("to_delete"),
      ..models::Namespace::default()
    };

    let mut post_resp = srv
    .post("/namespaces")
    .send_json(&test_namespace)
    .await.unwrap();

    let create_payload = post_resp
    .json::<responses::models::CreateResponse>()
    .await.unwrap();

    println!("create payload {:?}", create_payload);
    // create_payload.id;
    let mut response = srv
    .get(format!("/namespaces/{}", create_payload.id))
    .send()
    .await
    .unwrap();

    let status = response.status();
    let delete_payload = response.json::<responses::models::DeleteResponse>().await.unwrap();
    println!("delete_payload : {:?}", delete_payload);
    assert_eq!(status, StatusCode::ACCEPTED);
    assert_eq!(delete_payload.count, 1);
    Ok(())
  }
}
