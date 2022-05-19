use crate::datasources::mongo::{
  repository::Repository,
  datasource::DatasourceMongoDb
};
use crate::models::namespace::Namespace;

#[derive(Debug, Clone)]
pub struct Repositories {
  pub(crate) namespace: Repository<Namespace>,
}

#[derive(Debug, Clone)]
pub struct DaemonState {
  pub repositories: Repositories,
}

#[derive(Debug)]
pub struct AppStateError {
  pub message: String,
}

fn init_repositories(db: DatasourceMongoDb) -> Repositories {
  Repositories {
    namespace: db.new_repository::<Namespace>("namespace"),
  }
}

// Todo implement generic error //
pub async fn init_state() -> Result<DaemonState, AppStateError> {
  let database = match DatasourceMongoDb::connect().await {
    Ok(db) => db,
    Err(err) => {
      return Err(AppStateError {
        message: format!("mongo::connect error {}", &err),
      });
    },
  };

  let repositories = init_repositories(database);

  let state = DaemonState {
      // docker_api,
      repositories,
  };
  Ok(state)
}
