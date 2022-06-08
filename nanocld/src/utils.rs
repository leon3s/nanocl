use ntex::{http::StatusCode, web};

use crate::models::{DBConn, Pool};

use crate::controllers::errors::HttpError;

pub fn get_pool_conn(pool: &web::types::State<Pool>) -> Result<DBConn, HttpError> {
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

  use bollard::Docker;
  use crate::postgre::create_pool;
  
  pub use ntex::web::test::TestServer;

  pub type TestReturn = Result<(), Box<dyn std::error::Error + 'static>>;

  type Config = fn (&mut ServiceConfig);

  pub fn generate_server(config: Config) -> test::TestServer {
    let pool = create_pool();
    let docker = Docker::connect_with_socket_defaults().unwrap();
    test::server(move || App::new().state(docker.clone()).state(pool.clone()).configure(config))
  }
}
