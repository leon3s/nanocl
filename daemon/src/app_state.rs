use std::collections::HashMap;

use docker_api::Docker;
use docker_api::api::BuildOpts;
use docker_api::api::ImageBuildChunk;
use docker_api::api::PullOpts;
use docker_api::api::ContainerCreateOpts;
use docker_api::api::NetworkListOpts;
use futures::StreamExt;
use futures::TryStreamExt;
use serde_json::json;

use crate::docker;
use crate::datasources;
use crate::datasources::Repositories;
use crate::datasources::mongo::{DatasourceMongoDb, models};

#[derive(Debug, Clone)]
pub struct DaemonState {
  pub docker_api: Docker,
  pub repositories: Repositories,
}

#[derive(Debug)]
pub struct AppStateError {
  pub message: String,
}

fn init_repositories(db: DatasourceMongoDb) -> Repositories {
  Repositories {
    namespace: db.new_repository::<models::Namespace>("namespace"),
  }
}

// async fn download_mongodb_img(docker: &Docker) {
//   let mongodb_imgd = docker.images().get("mongodb").inspect().await;
//   if mongodb_imgd.is_ok() {
//     return;
//   }
//   println!("downloading mongodb docker image");
//   // docker.images().pull()
//   let pull_opts = PullOpts::builder().image("mongo").build();
//   let result = docker.images().pull(&pull_opts).try_collect::<Vec<ImageBuildChunk>>().await;
//   match result {
//     Ok(_) => {},
//     Err(err) => eprintln!("unable to download image {}", err),
//   }
//   // let res = stream.into_future().await;
//   println!("finished");
// }

async fn ensure_mongodb(docker: &Docker) {
  // download_mongodb_img(docker).await;
  // println!("{:?}", image);
}

pub async fn ensure_required_services(docker: &Docker) {
  // ensure_mongodb(docker).await;
  // let container_db = docker.containers().get("nanocldb").inspect().await;
  // if container_db.is_err() {
  //   println!("we have to create mongodb container for ours need.");
  //   let opts = ContainerCreateOpts::builder("mongo")
  //     .name("nanocldb")
  //     .env(vec!["MONGO_INITDB_ROOT_USERNAME", "root"])
  //     .env(vec!["MONGO_INITDB_ROOT_USERNAME", "root"])
  //     // .expose(1447, 1444)
  //     .build();
  //   let create_resp = docker.containers().create(&opts).await;
  //   println!("create_resp : {:?}", create_resp);
  //   if create_resp.is_ok() {
  //     println!("successfully created nanocldb container: {:?}", create_resp);
  //     let container = create_resp.unwrap();
  //     let res = container.start().await;
  //     println!("{:?}", res);
  //   }
  // }
  // println!("{:?}", container_db);
  // let networks = docker.networks().list(&NetworkListOpts::builder().build()).await;
}

// Todo implement generic error //
pub async fn init_state() -> Result<DaemonState, AppStateError> {
  let database = match datasources::mongo::connect().await {
    Ok(db) => db,
    Err(err) => {
      return Err(AppStateError {
        message: format!("mongo::connect error {}", &err),
      });
    },
  };

  let docker_api = match docker::new_docker() {
    Ok(docker) => docker,
    Err(err) => {
      return Err(AppStateError {
        message: format!("docker::new_docker error {:?}", &err),
      });
    }
  };

  let repositories = init_repositories(database);

  let state = DaemonState {
      docker_api,
      repositories,
  };
  Ok(state)
}
