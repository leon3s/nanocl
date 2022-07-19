//! File used to describe daemon boot
use std::collections::HashMap;

use bollard::models::{Ipam, IpamConfig};
use bollard::network::CreateNetworkOptions;
use ntex::web;

use bollard::errors::Error as DockerError;

use crate::config::DaemonConfig;
use crate::{services, repositories};
use crate::models::{Pool, NamespacePartial};

use crate::errors::DaemonError;

embed_migrations!("./migrations");

#[derive(Clone)]
pub struct BootState {
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
) -> Result<(), DaemonError> {
  const NSP_NAME: &str = "global";
  match repositories::namespace::inspect_by_name(NSP_NAME.to_string(), pool)
    .await
  {
    Err(_err) => {
      let new_nsp = NamespacePartial {
        name: NSP_NAME.to_string(),
      };
      repositories::namespace::create(new_nsp, pool).await?;
      Ok(())
    }
    Ok(_) => Ok(()),
  }
}

pub async fn create_default_network(
  docker_api: &bollard::Docker,
) -> Result<(), DockerError> {
  let network_name = "nanocl";
  let state =
    services::utils::get_network_state(docker_api, network_name).await?;
  if state == services::utils::NetworkState::NotFound {
    services::utils::create_network(docker_api, network_name).await?;
  }
  let mut options: HashMap<String, String> = HashMap::new();
  options.insert(
    String::from("com.docker.network.bridge.name"),
    String::from("nanoclvpn0"),
  );
  // let config = CreateNetworkOptions {
  //   name: String::from("nanocl-vpn"),
  //   driver: String::from("bridge"),
  //   options,
  //   ipam: Ipam {
  //     driver: Some(String::from("default")),
  //     config: Some(vec![IpamConfig {
  //       ip_range: Some(String::from("155.0.0.128/25")),
  //       subnet: Some(String::from("155.0.0.0/24")),
  //       gateway: Some(String::from("155.0.0.1")),
  //       ..Default::default()
  //     }]),
  //     ..Default::default()
  //   },
  //   ..Default::default()
  // };
  // docker_api.create_network(config).await?;
  Ok(())
}

async fn boot_docker_services(
  config: &DaemonConfig,
  docker_api: &bollard::Docker,
) -> Result<(), DaemonError> {
  create_default_network(docker_api).await?;
  // Boot postgresql service to ensure database connection
  services::postgresql::boot(config, docker_api).await?;
  // Boot dnsmasq service to manage domain names
  services::dnsmasq::boot(config, docker_api).await?;
  // Boot nginx service to manage proxy
  services::nginx::boot(config, docker_api).await?;
  services::ipsec::boot(config, docker_api).await?;
  Ok(())
}

/// Boot function called before http server start to
/// initialize his state and some background task
pub async fn boot(
  config: &DaemonConfig,
  docker_api: &bollard::Docker,
) -> Result<BootState, DaemonError> {
  log::info!("booting");
  boot_docker_services(config, docker_api).await?;
  let postgres_ip = services::postgresql::get_postgres_ip(docker_api).await?;
  log::info!("creating postgresql state pool");
  // Connect to postgresql
  let db_pool = services::postgresql::create_pool(postgres_ip.to_owned());
  let pool = web::types::State::new(db_pool.to_owned());
  let conn = services::postgresql::get_pool_conn(&pool)?;
  log::info!("runing migration");
  // wrap into state to be abble to use our functions
  embedded_migrations::run(&conn)?;
  // Create default namesapce
  create_default_nsp(&pool).await?;

  log::info!("booted");
  // Return state
  Ok(BootState {
    pool: db_pool,
    docker_api: docker_api.to_owned(),
  })
}
