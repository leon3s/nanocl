use ntex::web;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PingResponse {
    message: &'static str,
}

#[web::get("/ping")]
async fn get_ping(_req: web::HttpRequest) -> Result<web::HttpResponse, web::Error> {
    let response = PingResponse { message: "pong" };
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
  use crate::controllers::ping::*;
  use ntex::http::{StatusCode};
  use ntex::web::{test, App, Error};

  #[ntex::test]
  async fn test_get_ping() -> Result<(), Error> {
    let srv = test::server(move || {
        App::new()
        .configure(ctrl_config)
    });

    let mut response = srv
    .get("/ping")
    .send()
    .await
    .unwrap();

    let status = response.status();
    let body = response.body().await.unwrap();
    println!("{:?}", body);

    assert_eq!(status, StatusCode::OK);
    Ok(())
  }
}
