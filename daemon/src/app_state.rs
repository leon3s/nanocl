use docker_api::Docker;

use crate::{datasources::{self, mongo::{DatasourceMongoDb, models}, Repositories}, docker};

#[derive(Debug, Clone)]
pub struct DaemonState {
  pub database: DatasourceMongoDb,
  pub docker_api: Docker,
  pub repositories: Repositories,
}

#[derive(Debug)]
pub struct AppStateError {
  pub message: String,
}

pub fn init_repositories(db: &DatasourceMongoDb) -> Repositories {
  Repositories {
    namespace: db.new_repository::<models::Namespace>("namespace"),
  }
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

  let repositories = init_repositories(&database);

  let state = DaemonState {
      database,
      docker_api,
      repositories,
  };
  Ok(state)
}
