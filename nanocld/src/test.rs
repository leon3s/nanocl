#[cfg(test)]
pub mod utils {
  use ntex::web::*;

  use crate::postgre::create_pool;

  pub type TestReturn = Result<(), Box<dyn std::error::Error + 'static>>;

  type Config = fn (&mut ServiceConfig);

  pub fn generate_server(config: Config) -> test::TestServer {
    let pool = create_pool();
    test::server(move || App::new().state(pool.clone()).configure(config))
  }
}
