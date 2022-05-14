use mongodb::Database;
use docker_api::Docker;

#[derive(Debug, Clone)]
pub struct DaemonState {
  pub database: Database,
  pub docker_api: Docker,
}
