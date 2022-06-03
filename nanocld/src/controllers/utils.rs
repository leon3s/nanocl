use ntex::{http::StatusCode, web};

use crate::models::{DBConn, Pool};

use super::http_error::HttpError;

pub fn get_poll_conn(pool: web::types::State<Pool>) -> Result<DBConn, HttpError> {
    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return Err(HttpError {
                msg: String::from("unable to connect to nanocl-db"),
                status: StatusCode::INTERNAL_SERVER_ERROR,
            });
        }
    };
    Ok(conn)
}
