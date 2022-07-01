use ntex::web;
use ntex::http::StatusCode;

use crate::models::{DBConn, Pool};
use crate::controllers::errors::HttpError;

/// # Get connection from a pool
///
/// # Arguments
/// [pool](web::types::State<Pool>) a pool wrapped in ntex State
///
pub fn get_pool_conn(
  pool: &web::types::State<Pool>,
) -> Result<DBConn, HttpError> {
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

pub fn _get_free_port() -> Result<u16, HttpError> {
  let socket = match std::net::UdpSocket::bind("127.0.0.1:0") {
    Err(err) => {
      return Err(HttpError {
        msg: format!("unable to find a free port {:?}", err),
        status: StatusCode::INTERNAL_SERVER_ERROR,
      })
    }
    Ok(socket) => socket,
  };
  let port = match socket.local_addr() {
    Err(err) => {
      return Err(HttpError {
        msg: format!("unable to find a free port {:?}", err),
        status: StatusCode::INTERNAL_SERVER_ERROR,
      })
    }
    Ok(local_addr) => local_addr.port(),
  };
  drop(socket);
  Ok(port)
}

#[cfg(test)]
pub mod test {
  use ntex::web::*;

  use crate::{
    postgre::create_pool, services::postgresql::get_postgres_ip, models::Pool,
  };

  pub use ntex::web::test::TestServer;

  pub type TestReturn = Result<(), Box<dyn std::error::Error + 'static>>;

  type Config = fn(&mut ServiceConfig);

  pub fn gen_docker_client() -> bollard::Docker {
    bollard::Docker::connect_with_unix(
      "/run/nanocl/docker.sock",
      120,
      bollard::API_DEFAULT_VERSION,
    )
    .unwrap()
  }

  pub async fn gen_postgre_pool() -> Pool {
    let docker = gen_docker_client();
    let ip_addr = get_postgres_ip(&docker).await.unwrap();
    let pool = create_pool(ip_addr);
    pool
  }

  pub async fn generate_server(config: Config) -> test::TestServer {
    let docker = gen_docker_client();

    let ip_addr = get_postgres_ip(&docker).await.unwrap();

    let pool = create_pool(ip_addr);
    test::server(move || {
      App::new()
        .state(pool.clone())
        .state(docker.clone())
        .configure(config)
    })
  }
}
