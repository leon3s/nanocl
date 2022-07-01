//! Custom ntex http error to serialize error as json
use ntex::web;
use ntex::http::StatusCode;
use serde::Serialize;
use serde_json::{json, to_string_pretty};
use std::fmt::{Display, Formatter, Result as FmtResult};
use utoipa::Component;

#[derive(Debug, Serialize)]
pub struct HttpError {
  pub(crate) msg: String,
  #[serde(skip_serializing)]
  pub(crate) status: StatusCode,
}

#[derive(Component)]
pub struct ApiError {
  #[allow(dead_code)]
  pub(crate) msg: String,
}

impl Display for HttpError {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", to_string_pretty(self).unwrap())
  }
}

pub trait IntoHttpError {
  fn to_http_error(&self) -> HttpError;
}

impl web::WebResponseError for HttpError {
  // builds the actual response to send back when an error occurs
  fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
    log::error!("[{}] {}", self.status, self.msg);
    let err_json = json!({ "msg": self.msg });
    web::HttpResponse::build(self.status).json(&err_json)
  }
}
