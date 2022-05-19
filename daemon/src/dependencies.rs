use futures::StreamExt;
use bollard::{Docker, image::CreateImageOptions, container::{CreateContainerOptions, Config}};

async fn download_mongo_image(docker: &Docker) {
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
      Ok(output) => println!("{:?}", output),
      Err(err) => panic!("{:?}", err),
    }
  }
}

pub async fn init_mongo_container(docker: &Docker) {
  download_mongo_image(docker).await;
  let options = Some(CreateContainerOptions{
    name: "my-new-container",
  });

  let config = Config {
      image: Some("hello-world"),
      cmd: Some(vec!["/hello"]),
      env: Some(vec!["test=gg"]),
      ..Default::default()
  };
  let result = match docker.create_container(options, config).await {
    Ok(result) => result,
    Err(err) => panic!("{:?}", err),
  };
  // docker.start_container() 
}
