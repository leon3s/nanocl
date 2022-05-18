use ntex::web;
use serde::{Serialize, Deserialize};

mod docker;
mod responses;
mod app_state;
mod datasources;
mod controllers;

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
    let state = match app_state::init_state().await {
        Ok(state) => state,
        Err(err) => panic!("Error while initing application state {}", err.message),
    };
    // let mut server = server::create_server();
    let mut server = web::HttpServer::new(move ||
        web::App::new()
        .wrap(web::middleware::Logger::default())
        .state(state.clone())
        .default_service(
            web::route().to(default_response)
        )
        .configure(controllers::ping::ctrl_config)
        .configure(controllers::system::ctrl_config)
        .configure(controllers::namespace::ctrl_config)
        .configure(controllers::docker_image::ctrl_config)
    );
    server = server.bind("0.0.0.0:8383")?;
    println!("starting server on 0.0.0.0:8383");
    server.run().await
}
