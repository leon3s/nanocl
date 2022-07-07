use ntex::web;
use ntex::{http::StatusCode, web::error::BlockingError};

use crate::errors::HttpResponseError;

/// Convert diesel::result::Error into HttpResponseError
///
/// # Arguments
///
/// * `err` - Diesel result error
///
/// # Examples
///
/// ```
/// // Return Error
///
/// use crate::repositories::errors::db_error;
/// Err(db_error(err))
/// ```
pub fn db_error(err: diesel::result::Error) -> HttpResponseError {
  println!("got db error : {:#?}", err);
  let default_error = HttpResponseError {
    msg: String::from("unproccesable query"),
    status: StatusCode::BAD_REQUEST,
  };
  match err {
    diesel::result::Error::InvalidCString(_) => default_error,
    diesel::result::Error::DatabaseError(_, _) => default_error,
    diesel::result::Error::NotFound => HttpResponseError {
      msg: String::from("item not found"),
      status: StatusCode::NOT_FOUND,
    },
    diesel::result::Error::QueryBuilderError(_) => default_error,
    diesel::result::Error::DeserializationError(_) => default_error,
    diesel::result::Error::SerializationError(_) => default_error,
    diesel::result::Error::RollbackTransaction => default_error,
    diesel::result::Error::AlreadyInTransaction => default_error,
    _ => HttpResponseError {
      msg: String::from("unexpected error"),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    },
  }
}

/// Convert BlockingError<diesel::result::Error> into HttpResponseError
///
/// # Arguments
///
/// * `err` - BlockingError diesel result error
///
/// # Examples
///
/// ```
/// // Return Error
///
/// use crate::repositories::errors::db_blocking_error;
/// Err(db_blocking_error(err))
/// ```
pub fn db_blocking_error(
  err: BlockingError<diesel::result::Error>,
) -> HttpResponseError {
  match err {
    web::error::BlockingError::Error(db_err) => db_error(db_err),
    web::error::BlockingError::Canceled => HttpResponseError {
      msg: String::from("unexpected error"),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    },
  }
}
