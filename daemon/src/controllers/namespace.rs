use ntex::web;
use mongodb::bson::doc;

use serde::{Serialize, Deserialize};

use crate::app_state::DaemonState;

use crate::models::errors;
use crate::models::namespace::{Namespace, NamespaceCreate};
use crate::models::generic::{CreateResponse, DeleteResponse};

#[utoipa::path(
  get,
  path = "/namespaces",
  responses(
      (status = 200, description = "Array of namespace found", body = Namespace),
  ),
)]
#[web::get("/namespaces")]
pub async fn list_namespace(
  state: web::types::State<DaemonState>,
) -> Result<web::HttpResponse, errors::HttpError> {
    let namespace = &state.repositories.namespace;
    let resp = match namespace.find().await {
      Ok(response) => response,
      Err(err) => {
        eprintln!("mongo error {}", err);
        return Err(
          errors::mongo_error(err)
        );
      },
    };
    println!("server side resp {:?}", resp);
    Ok(
      web::HttpResponse::Ok()
      .content_type("application/json")
      .json(&resp)
    )
}

#[web::post("/namespaces")]
async fn create_namespace(
  state: web::types::State<DaemonState>,
  payload: web::types::Json<NamespaceCreate>
) -> Result<web::HttpResponse, errors::HttpError> {
  let namespace = &state.repositories.namespace;
  let new_namespace = Namespace {
    name: payload.name.clone(),
    ..Namespace::default()
  };
  let id = match namespace.create(new_namespace).await {
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

#[derive(Debug, Serialize, Deserialize)]
struct MongoWhereLike {
  like: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum MongoWhereProperty {
  None,
  String(String),
  Like(MongoWhereLike),
}

#[derive(Debug, Serialize, Deserialize)]
struct NamespaceWhere {
  name: Option<MongoWhereProperty>,
}

#[derive(Debug, Deserialize)]
struct DeleteQuery {
  m_where: Option<String>,
}

// #[web::get("/namespaces/query")]
// async fn delete_namespace(
//   state: web::types::State<DaemonState>,
//   web::types::Query(query): web::types::Query<DeleteQuery>
// ) -> Result<web::HttpResponse, errors::HttpError> {
//   let namespace = &state.repositories.namespace;
//   println!("query : {:?}", query.m_where);
//   let delete_query = match query.m_where {
//     Some(json_str) => {
//       match serde_json::from_str::<NamespaceWhere>(&json_str) {
//         Ok(json) => {
//           println!("json {:?}", json);
//           match to_document(&json) {
//             Ok(doc) => doc,
//             Err(err) => {
//               return Err(errors::HttpError {
//                 status: 400,
//                 msg: format!("struct to mongo document error -> {}", err),
//               });
//             }
//           }
//         },
//         Err(err) => {
//           return Err(errors::HttpError {
//             status: 400,
//             msg: format!("query m_where parse fail -> {}", err),
//           });
//         }
//       }
//     }
//     None => doc! {},
//   };
//   println!("delete_query {:?}", delete_query);
//   let count = match namespace.delete(delete_query).await {
//     Ok(count) => count,
//     Err(err) => {
//       return Err(errors::mongo_error(err));
//     }
//   };
//   Ok(
//     web::HttpResponse::Accepted()
//     .content_type("application/json")
//     .json(&DeleteResponse {
//       count
//     })
//   )
// }

#[web::delete("/namespaces/{id}")]
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
  config.service(list_namespace);
  config.service(create_namespace);
  // config.service(delete_namespace);
  config.service(delete_namespace_by_id);
}

#[cfg(test)]
mod ctrl_namespace_tests {
  use ntex::http::StatusCode;
  use ntex::web::{test, App, Error};

  use crate::{app_state, models::namespace::Namespace};
  use crate::controllers::namespace::*;

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
    .send_json(&Namespace {
      name: "test".to_string(),
      ..Namespace::default()
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
    let _resp_body = resp.json::<Vec<Namespace>>().await.unwrap();
    assert_eq!(resp_status, StatusCode::OK);
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

    let test_namespace = Namespace {
      name: String::from("to_delete"),
      ..Namespace::default()
    };

    let mut post_resp = srv
    .post("/namespaces")
    .send_json(&test_namespace)
    .await.unwrap();

    let create_payload = post_resp
    .json::<CreateResponse>()
    .await.unwrap();
    // create_payload.id;
    let mut response = srv
    .delete(format!("/namespaces/{}", create_payload.id))
    .send()
    .await
    .unwrap();

    let status = response.status();
    let delete_payload = response.json::<DeleteResponse>().await.unwrap();
    assert_eq!(status, StatusCode::ACCEPTED);
    assert_eq!(delete_payload.count, 1);
    Ok(())
  }
}
