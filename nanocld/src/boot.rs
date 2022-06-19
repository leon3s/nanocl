//! File used to describe daemon boot
use ntex::web;

use crate::postgre;
use crate::services;
use crate::repositories;
use crate::controllers::errors::HttpError;
use bollard::errors::Error as DockerError;
use crate::models::{Pool, NamespacePartial};

#[derive(Debug)]
pub enum BootError {
  Errorhttp(HttpError),
  Errordocker(DockerError),
}

#[derive(Clone)]
pub struct DaemonState {
  pub(crate) pool: Pool,
  pub(crate) docker: bollard::Docker,
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
  const NSP_NAME: &str = "default";
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
  create_default_network(docker)
    .await
    .map_err(BootError::Errordocker)?;
  // Boot postgresql service to ensure database connection
  services::postgresql::boot(docker)
    .await
    .map_err(BootError::Errordocker)?;

  // Boot dnsmasq service to manage domain names
  services::dnsmasq::boot(docker)
    .await
    .map_err(BootError::Errordocker)?;

  // Boot nginx service to manage proxy
  services::nginx::boot(docker)
    .await
    .map_err(BootError::Errordocker)?;
  Ok(())
}

/// Boot function called before server start to initialize his state
pub async fn boot() -> Result<DaemonState, BootError> {
  // Boot services
  let docker = bollard::Docker::connect_with_unix(
    "/run/nanocl/docker.sock",
    120,
    bollard::API_DEFAULT_VERSION,
  )
  .map_err(BootError::Errordocker)?;
  boot_docker_services(&docker).await?;

  // Connect to postgresql
  let db_pool = postgre::create_pool();
  let pool = web::types::State::new(db_pool.clone());

  // Create default namesapce
  create_default_nsp(&pool).await?;

  // Return state
  Ok(DaemonState {
    pool: db_pool,
    docker,
  })
}
