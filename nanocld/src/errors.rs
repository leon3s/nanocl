use ntex::web;
use ntex::http::StatusCode;
use serde_json::json;
use thiserror::Error;

use bollard::errors::Error as DockerError;
use diesel_migrations::RunMigrationsError;
#[cfg(feature = "openapi")]
use utoipa::Component;

/// Http response error
#[derive(Debug, Error)]
pub struct HttpResponseError {
  pub(crate) msg: String,
  pub(crate) status: StatusCode,
}

impl std::fmt::Display for HttpResponseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}] {}", self.status, self.msg)
  }
}

pub trait IntoHttpResponseError {
  fn to_http_error(&self) -> HttpResponseError;
}

impl web::WebResponseError for HttpResponseError {
  // builds the actual response to send back when an error occurs
  fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
    log::error!("Error response: {}", self);
    let err_json = json!({ "msg": self.msg });
    web::HttpResponse::build(self.status).json(&err_json)
  }
}

/// Api Error Structure that server send to client
#[cfg_attr(feature = "openapi", derive(Component))]
#[allow(dead_code)]
pub struct ApiError {
  pub(crate) msg: String,
}

/// Generic Daemon error
#[derive(Debug, Error)]
pub enum DaemonError {
  /// Docker api error
  #[error(transparent)]
  Docker(#[from] DockerError),
  /// Diesel migration error
  #[error(transparent)]
  DieselMigration(#[from] RunMigrationsError),
  /// HttpResponseError
  #[error(transparent)]
  HttpResponse(#[from] HttpResponseError),
}
