use ntex::web;
use ntex::{http::StatusCode, web::error::BlockingError};

use crate::controllers::errors::HttpError;

pub fn db_error(err: diesel::result::Error) -> HttpError {
  let default_error = HttpError {
    msg: String::from("unproccesable query"),
    status: StatusCode::BAD_REQUEST,
  };
  match err {
    diesel::result::Error::InvalidCString(_) => default_error,
    diesel::result::Error::DatabaseError(_, _) => default_error,
    diesel::result::Error::NotFound => HttpError {
        msg: String::from("item not found"),
        status: StatusCode::NOT_FOUND,
    },
    diesel::result::Error::QueryBuilderError(_) => default_error,
    diesel::result::Error::DeserializationError(_) => default_error,
    diesel::result::Error::SerializationError(_) => default_error,
    diesel::result::Error::RollbackTransaction => default_error,
    diesel::result::Error::AlreadyInTransaction => default_error,
    _ => HttpError {
        msg: String::from("unexpected error"),
        status: StatusCode::INTERNAL_SERVER_ERROR,
    },
  }
}

// todo generic database error
pub fn db_blocking_error(err: BlockingError<diesel::result::Error>) -> HttpError {
  match err {
      web::error::BlockingError::Error(db_err) => {
        db_error(db_err)
      }
      web::error::BlockingError::Canceled => HttpError {
          msg: String::from("unexpected error"),
          status: StatusCode::INTERNAL_SERVER_ERROR,
      },
  }
}
