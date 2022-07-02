use ntex::http::StatusCode;

use crate::controllers::errors::HttpError;

pub fn docker_error_ref(err: &bollard::errors::Error) -> HttpError {
  match err {
    bollard::errors::Error::DockerResponseServerError {
      status_code,
      message,
    } => HttpError {
      msg: message.to_owned(),
      status: StatusCode::from_u16(status_code.to_owned()).unwrap(),
    },
    bollard::errors::Error::JsonDataError { message, .. } => HttpError {
      msg: message.to_owned(),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    },
    _ => HttpError {
      msg: format!("unexpected docker api error {:#?}", err),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    },
  }
}

pub fn docker_error(err: bollard::errors::Error) -> HttpError {
  docker_error_ref(&err)
}
