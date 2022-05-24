use ntex::{web, http::StatusCode};

use crate::models::{Pool, DBConn};

use super::http_error::HttpError;

pub fn get_poll_conn(pool: web::types::State<Pool>) -> Result<DBConn, HttpError> {
  let conn = match pool.get() {
    Ok(conn) => conn,
    Err(_) => {
      return Err(HttpError {
        msg: String::from(""),
        status: StatusCode::INTERNAL_SERVER_ERROR,
      });
    },
  };
  Ok(conn)
}