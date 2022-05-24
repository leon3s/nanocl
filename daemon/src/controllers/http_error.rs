use ntex::web;
use serde::Serialize;
use serde_json::{json, to_string_pretty};
use ntex::{http::StatusCode, web::error::BlockingError};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Serialize)]
pub struct HttpError {
    pub(crate) msg: String,
    #[serde(skip_serializing)]
    pub(crate) status: StatusCode,
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
        web::HttpResponse::build(self.status)
            .json(&err_json)
    }
}

// todo generic database error
pub fn db_bloking_error(err: BlockingError<diesel::result::Error>) -> HttpError {
  match err {
    web::error::BlockingError::Error(db_err) => {
      let default_error = HttpError {
        msg: String::from("unproccesable query"),
        status: StatusCode::BAD_REQUEST,
      };
      match db_err {
        diesel::result::Error::InvalidCString(_) => default_error,
        diesel::result::Error::DatabaseError(_, _) => {
          default_error
        },
        diesel::result::Error::NotFound => {
          HttpError {
            msg: String::from("item not found"),
            status: StatusCode::NOT_FOUND,
          }
        },
        diesel::result::Error::QueryBuilderError(_) => default_error,
        diesel::result::Error::DeserializationError(_) => default_error,
        diesel::result::Error::SerializationError(_) => default_error,
        diesel::result::Error::RollbackTransaction => default_error,
        diesel::result::Error::AlreadyInTransaction => default_error,
        _ => {
          HttpError {
            msg: String::from("unexpected error"),
            status: StatusCode::INTERNAL_SERVER_ERROR,
          }
        },
      }
    },
    web::error::BlockingError::Canceled => {
      HttpError {
        msg: String::from("unexpected error"),
        status: StatusCode::INTERNAL_SERVER_ERROR,
      }
    },
  }
}