use futures::TryStreamExt;
use ntex::rt;
use ntex::http::client::{Client, Connector};
use bollard::{
  Docker,
  container::{StatsOptions, Stats},
};

mod nginx;
mod deploy;
mod dnsmasq;
mod network;
mod posgresql;
mod docker_helper;

type _Callback = fn(stats: Stats);

async fn _test_stats(docker: &Docker, callback: _Callback) {
  let options = Some(StatsOptions {
    stream: false,
    one_shot: false,
  });
  let mut stream = docker.stats("nanoclq", options);
  let stats = stream.try_next().await;
  match stats {
    Ok(stats) => match stats {
      Some(stats) => {
        println!("{:?}", stats);
        callback(stats);
      }
      None => {
        eprintln!("Stats are empty");
      }
    },
    Err(err) => {
      eprintln!("error while collecting stats {}", err);
    }
  }
}

async fn init_services(docker: &Docker) {
  if let Err(err) = network::ensure_start(docker).await {
    panic!("unable to setup nanocl network {}", err);
  }
  if let Err(err) = dnsmasq::ensure_start(docker).await {
    panic!("unable to setup dnsmasq service {}", err);
  }
  if let Err(err) = nginx::ensure_start(docker).await {
    panic!("unable to setup nginx service {}", err);
  }
  if let Err(err) = posgresql::ensure_start(docker).await {
    panic!("unable to setup postgresql service {}", err);
  }
}

#[ntex::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
  // let docker = Docker::connect_with_socket_defaults()?;
  // init_services(&docker).await;
  let client = Client::build()
    .connector(
      Connector::default()
        .connector(ntex::service::fn_service(|_| async {
          Ok(rt::unix_connect("/var/run/docker.sock").await?)
        }))
        .finish(),
    )
    .finish();

  let res = client
    .get("http://localhost/containers/nanocl-db-postgre/stats")
    .send()
    .await?;

  let mut stream = res.into_stream();
  while let Some(body) = stream.try_next().await.unwrap() {
    println!("body : {:?}", body);
  }

  Ok(())
}
