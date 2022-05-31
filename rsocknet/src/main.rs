use ntex::web;
use ntex_files as fs;

#[ntex::main]
async fn main() -> std::io::Result<()> {
  let mut server = web::HttpServer::new(|| {
    web::App::new()
    .service(
      fs::Files::new(
        "/",
        "./static")
      .index_file("websocket.html"),
    )
  });
  server = server.bind("0.0.0.0:3000")?;
  println!("starting server on http://0.0.0.0:3000");
  server.run().await
}
