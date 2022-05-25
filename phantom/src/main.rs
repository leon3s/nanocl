use futures::TryStreamExt;
use bollard::{
  Docker,
  container::{
    StatsOptions,
    Stats,
  },
};

mod nginx;
mod deploy;
mod posgresql;
mod docker_helper;

type _Callback = fn (stats: Stats);

async fn _test_stats(docker: &Docker, callback: _Callback) {
  let options = Some(StatsOptions {
    stream: false,
    one_shot: false,
  });
  let mut stream = docker.stats("nanoclq", options);
  let stats = stream.try_next().await;
  match stats {
    Ok(stats) => {
      match stats {
        Some(stats) => {
          println!("{:?}", stats);
          callback(stats);
        },
        None => {
          eprintln!("Stats are empty");
        }
      }
    },
    Err(err) => {
      eprintln!("error while collecting stats {}", err);
    }
  }
  println!("finished .");
}

#[ntex::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
  let docker = Docker::connect_with_socket_defaults().unwrap();

  nginx::ensure_start(&docker).await;
  posgresql::ensure_start(&docker).await;
  Ok(())
}
