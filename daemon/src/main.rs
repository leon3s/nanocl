use ntex::web;

mod controllers;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    // Enable ntex logs
    // std::env::set_var("RUST_LOG", "ntex=trace");
    env_logger::init();
    // let mut server = server::create_server();
    let mut server = web::HttpServer::new(||
        web::App::new()
        .wrap(web::middleware::Logger::default())
        .configure(controllers::ping::ctrl_config)
        .configure(controllers::system::ctrl_config)
    );
    server = server.bind("0.0.0.0:8383")?;
    println!("starting server on 0.0.0.0:8383");
    server.run().await
}
