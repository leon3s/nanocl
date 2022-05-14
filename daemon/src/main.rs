mod datasource;
mod app_state;
mod controllers;

use ntex::web;
use app_state::DaemonState;
use docker_api::Docker;
use datasource::mongo_connect;
use serde::{Serialize, Deserialize};

#[cfg(unix)]
pub fn new_docker() -> Result<Docker, ()> {
    Ok(Docker::unix("/var/run/docker.sock"))
}

#[cfg(not(unix))]
pub fn new_docker() -> Result<Docker, ()> {
    Docker::new("tcp://127.0.0.1:8080")
}

#[derive(Serialize, Deserialize, Debug)]
struct DefaultResponse {
    pub message: String,
}

async fn default_response() -> Result<web::HttpResponse, web::Error> {
    let resp = DefaultResponse {
        message: String::from("not found."),
    };
    Ok(web::HttpResponse::NotFound().json(&resp))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    // Enable ntex logs
    // std::env::set_var("RUST_LOG", "ntex=trace");
    env_logger::init();
    let database = match mongo_connect().await {
        Ok(database) => database,
        Err(err) => panic!("mongo_connect error : {:?}", &err),
    };
    let docker_api = match new_docker() {
        Ok(api) => api,
        Err(_) => panic!("Not abble to connect to docker"),
    };
    let state = DaemonState {
        database,
        docker_api,
    };
    // let mut server = server::create_server();
    let mut server = web::HttpServer::new(move ||
        web::App::new()
        .default_service(
            web::route().to(default_response)
        ).app_state(state.clone())
        .wrap(web::middleware::Logger::default())
        .configure(controllers::ping::ctrl_config)
        .configure(controllers::system::ctrl_config)
    );
    server = server.bind("0.0.0.0:8383")?;
    println!("starting server on 0.0.0.0:8383");
    server.run().await
}
