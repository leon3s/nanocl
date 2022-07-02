//! File used to describe daemon boot
use ntex::web;

use crate::{services, repositories};
use crate::models::{Pool, NamespacePartial};
use crate::controllers::errors::HttpError;

use bollard::errors::Error as DockerError;
use diesel_migrations::RunMigrationsError;

embed_migrations!("./migrations");

#[derive(Debug)]
pub enum BootError {
  Errorhttp(HttpError),
  Errordocker(DockerError),
  Errormigration(RunMigrationsError),
}

#[derive(Debug)]
#[allow(dead_code)]
/// Todo Daemon config as state
pub struct DaemonConfig {
  root_path: String,
  state_path: String,
  pidfile: String,
  hosts: Vec<String>,
}

#[derive(Clone)]
pub struct DaemonState {
  pub(crate) pool: Pool,
  pub(crate) docker_api: bollard::Docker,
}

/// # Create default namespace
/// Create a namespace with default as name if he doesn't exist
///
/// # Arguments
/// - [pool](web::types::State<Pool>) Postgres database pool
///
/// # Examples
/// ```rust,norun
/// create_default_nsp(&pool).await;
/// ```
async fn create_default_nsp(
  pool: &web::types::State<Pool>,
) -> Result<(), BootError> {
  const NSP_NAME: &str = "global";
  match repositories::namespace::inspect_by_name(NSP_NAME.to_string(), pool)
    .await
  {
    Err(_err) => {
      let new_nsp = NamespacePartial {
        name: NSP_NAME.to_string(),
      };
      match repositories::namespace::create(new_nsp, pool).await {
        Err(err) => Err(BootError::Errorhttp(err)),
        Ok(_nsp) => Ok(()),
      }
    }
    Ok(_) => Ok(()),
  }
}

pub async fn create_default_network(
  docker: &bollard::Docker,
) -> Result<(), DockerError> {
  let network_name = "nanocl";
  let state = services::utils::get_network_state(docker, network_name).await?;
  if state == services::utils::NetworkState::NotFound {
    services::utils::create_network(docker, network_name).await?;
  }
  Ok(())
}

async fn boot_docker_services(
  docker: &bollard::Docker,
) -> Result<(), BootError> {
  log::info!("ensuring nanocl network");
  create_default_network(docker)
    .await
    .map_err(BootError::Errordocker)?;
  log::info!("ensuring postgresql boot");
  // Boot postgresql service to ensure database connection
  services::postgresql::boot(docker)
    .await
    .map_err(BootError::Errordocker)?;

  log::info!("ensuring dnsmasq boot");
  // Boot dnsmasq service to manage domain names
  services::dnsmasq::boot(docker)
    .await
    .map_err(BootError::Errordocker)?;

  log::info!("ensuring nginx boot");
  // Boot nginx service to manage proxy
  services::nginx::boot(docker)
    .await
    .map_err(BootError::Errordocker)?;
  Ok(())
}

/// Boot function called before server start to initialize his state
pub async fn boot() -> Result<DaemonState, BootError> {
  // Boot services
  log::info!("booting");
  log::info!("connecting to docker on /run/nanocl/docker.sock");
  let docker_api = bollard::Docker::connect_with_unix(
    "/run/nanocl/docker.sock",
    120,
    bollard::API_DEFAULT_VERSION,
  )
  .map_err(BootError::Errordocker)?;
  boot_docker_services(&docker_api).await?;
  // Connect to postgresql
  let postgres_ip = services::postgresql::get_postgres_ip(&docker_api)
    .await
    .map_err(BootError::Errorhttp)?;
  log::info!("creating postgresql state pool");
  let db_pool = services::postgresql::create_pool(postgres_ip.to_owned());
  let pool = web::types::State::new(db_pool.to_owned());
  log::info!("creating postgresql migration pool");
  let conn =
    services::postgresql::get_pool_conn(&pool).map_err(BootError::Errorhttp)?;
  // wrap into state to be abble to use our functions
  log::info!("running migration script");
  embedded_migrations::run(&conn).map_err(BootError::Errormigration)?;
  // Create default namesapce
  log::info!("ensuring namespace 'global' presence");
  create_default_nsp(&pool).await?;

  log::info!("booted");
  // Return state
  Ok(DaemonState {
    pool: db_pool,
    docker_api,
  })
}

#[cfg(test)]
mod test_boot {

  use super::boot;

  #[ntex::test]
  async fn test_boot() {
    boot().await.unwrap();
  }
}
