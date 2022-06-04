use ntex::{http::StatusCode, web};

use crate::models::{DBConn, Pool};

use crate::controllers::errors::HttpError;

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

#[cfg(test)]
pub mod test {
  use ntex::web::*;

  use crate::postgre::create_pool;

  pub type TestReturn = Result<(), Box<dyn std::error::Error + 'static>>;

  type Config = fn (&mut ServiceConfig);

  pub fn generate_server(config: Config) -> test::TestServer {
    let pool = create_pool();
    test::server(move || App::new().state(pool.clone()).configure(config))
  }
}
