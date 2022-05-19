use ntex::web;
use serde::{Serialize, Deserialize};

use crate::models::errors;

#[derive(Serialize, Deserialize, Debug)]
pub struct PingResponse {
    message: String,
}

#[web::get("/ping")]
async fn get_ping() -> Result<web::HttpResponse, errors::HttpError> {
    let response = PingResponse {
      message: String::from("pong"),
    };
    Ok(
        web::HttpResponse::Ok()
        .content_type("application/json")
        .json(&response)
    )
}

pub fn ctrl_config(config: &mut web::ServiceConfig) {
  config.service(get_ping);
}

#[cfg(test)]
mod ctrl_ping_tests {
  use ntex::http::StatusCode;
  use ntex::web::{test, App, Error};
  use crate::app_state;
  use crate::controllers::ping::*;

  #[ntex::test]
  async fn test_get_ping() -> Result<(), Error> {
    let state = app_state::init_state().await.unwrap();
    let srv = test::server(move || {
        App::new()
        .app_state(state.clone())
        .configure(ctrl_config)
    });

    let mut response = srv
    .get("/ping")
    .send()
    .await
    .unwrap();

    let status = response.status();
    let _body = response.body().await.unwrap();
    assert_eq!(status, StatusCode::OK);
    Ok(())
  }
}
