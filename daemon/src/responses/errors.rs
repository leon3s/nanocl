use std::fmt::{Display, Formatter, Result as FmtResult};
use mongodb::error::Error as MongoError;
use docker_api::Error as DockerError;

use ntex::web;
use ntex::http::StatusCode;
use serde::Serialize;
use serde_json::{json, to_string_pretty};

#[derive(Debug, Serialize)]
pub struct HttpError {
    msg: String,
    status: u16,
}

impl HttpError {
  fn new(status: u16, msg: String) -> HttpError {
    HttpError { status, msg }
  }
}

impl Display for HttpError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl web::WebResponseError for HttpError {
    // builds the actual response to send back when an error occurs
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        let err_json = json!({ "error": self.msg });
        web::HttpResponse::build(StatusCode::from_u16(self.status).unwrap())
            .json(&err_json)
    }
}

// Todo generic mongo errors
pub fn mongo_error(_error: MongoError) -> HttpError {
  HttpError::new(
    500,
    String::from("unexpected mongo error"),
  )
}

// Todo generic docker errors
pub fn docker_error(_err: DockerError) -> HttpError {
  HttpError::new(
    500,
    String::from("unexpected docker error"),
  )
}
