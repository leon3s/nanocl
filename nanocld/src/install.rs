//! File used to describe daemon boot

use crate::services;
use crate::errors::DaemonError;

pub async fn install_services(
  docker: &bollard::Docker,
) -> Result<(), DaemonError> {
  services::utils::install_service("postgres:latest", docker).await?;
  services::utils::install_service("hwdsl2/ipsec-vpn-server", docker).await?;
  services::utils::build_service("nanocl-dns-dnsmasq", docker).await?;
  services::utils::build_service("nanocl-proxy-nginx", docker).await?;

  Ok(())
}
