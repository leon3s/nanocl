#[macro_use]
extern crate diesel;

use bollard::Docker;
use ntex::web;
use ntex_files as fs;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

mod utils;
mod models;
mod schema;
mod openapi;
mod controllers;
mod repositories;

#[ntex::main]
async fn main() -> std::io::Result<()> {
  env_logger::init();

  let db_url = "postgres://root:root@nanocl-db-postgre/nanocl";
  let manager = ConnectionManager::<PgConnection>::new(db_url);
  let pool = r2d2::Pool::builder()
      .build(manager)
      .expect("Failed to create pool.");

  let docker = Docker::connect_with_socket_defaults().unwrap();

  let mut server = web::HttpServer::new(move ||
    web::App::new()
    .state(docker.clone())
    .state(pool.clone())
    .wrap(web::middleware::Logger::default())
    .app_state(web::types::JsonConfig::default().limit(4096))
    .configure(
      openapi::ntex_config
    )
    .configure(
      controllers::namespace::ntex_config
    )
    .configure(
      controllers::namespace_git_repository::ntex_config
    )
    .configure(
      controllers::container::ntex_config
    )
    .service(
      fs::Files::new(
        "/websocket",
        "./static/websocket")
      .index_file("index.html")
    )
  );
  server = server.bind("0.0.0.0:8383")?;
  println!("starting server on http://0.0.0.0:8383");
  server.run().await
}
