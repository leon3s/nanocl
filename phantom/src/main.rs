// use docker_api::{Docker, api::PullOpts};
use bollard::Docker;
use bollard::image::CreateImageOptions;
use futures_util::{TryStreamExt, StreamExt};

#[ntex::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
  // let docker = Docker::unix("/var/run/docker.sock");
  // let pull_opts = PullOpts::builder().image("mongo").build();
  // let mut stream = docker.images().pull(&pull_opts);
  // while let Some(ouput) = stream.next().await {
  //   match ouput {
  //     Ok(ouput) => println!("{:?}", ouput),
  //     Err(err) => eprintln!("{:?}", err),
  //   }
  // }
  // println!("gg");
  let docker = Docker::connect_with_socket_defaults().unwrap();
  let mut stream = docker
  .create_image(
      Some(CreateImageOptions {
          from_image: "mongo:latest",
          ..Default::default()
      }),
      None,
      None,
  );
  while let Some(output) = stream.next().await {
    match output {
      Ok(o) => println!("{:?}", o),
      Err(err) => eprintln!("error : {}", err),
    }
  }
  // .try_collect::<Vec<_>>()
  // .await?;

  println!("finished.");
  // docker.create_image();
  Ok(())
}
