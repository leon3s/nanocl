use std::collections::HashMap;
use std::path::Path;

use bollard::Docker;
use bollard::container::{CreateContainerOptions, Config};
use bollard::errors::Error as DockerError;
use bollard::models::{HostConfig, PortBinding, DeviceMapping};

use crate::config::DaemonConfig;

use super::utils::*;

fn gen_ipsec_host_conf(config: &DaemonConfig) -> HostConfig {
  let path = Path::new(&config.state_dir).join("ipsec");

  let binds = vec![
    format!("{}:/etc/ipsec.d", path.display()),
    String::from("/lib/modules:/lib/modules:ro"),
  ];
  let mut port_bindings: HashMap<String, Option<Vec<PortBinding>>> =
    HashMap::new();
  let mut sysctl: HashMap<String, String> = HashMap::new();
  port_bindings.insert(
    String::from("4500/udp"),
    Some(vec![PortBinding {
      host_ip: None,
      host_port: Some(String::from("4500/udp")),
    }]),
  );
  port_bindings.insert(
    String::from("500/udp"),
    Some(vec![PortBinding {
      host_ip: None,
      host_port: Some(String::from("500/udp")),
    }]),
  );
  sysctl.insert(String::from("net.ipv4.ip_forward"), String::from("1"));
  sysctl.insert(
    String::from("net.ipv4.conf.all.accept_redirects"),
    String::from("0"),
  );
  sysctl.insert(
    String::from("net.ipv4.conf.all.send_redirects"),
    String::from("0"),
  );
  sysctl.insert(
    String::from("net.ipv4.conf.all.rp_filter"),
    String::from("0"),
  );
  sysctl.insert(
    String::from("net.ipv4.conf.default.accept_redirects"),
    String::from("0"),
  );
  sysctl.insert(
    String::from("net.ipv4.conf.default.send_redirects"),
    String::from("0"),
  );
  sysctl.insert(
    String::from("net.ipv4.conf.default.rp_filter"),
    String::from("0"),
  );
  sysctl.insert(
    String::from("net.ipv4.conf.eth0.send_redirects"),
    String::from("0"),
  );
  sysctl.insert(
    String::from("net.ipv4.conf.eth0.rp_filter"),
    String::from("0"),
  );

  HostConfig {
    binds: Some(binds),
    privileged: Some(true),
    port_bindings: Some(port_bindings),
    // network_mode: Some(String::from("host")),
    // cap_add: Some(vec![String::from("NET_ADMIN")]),
    // sysctls:
    // devices: Some(vec![DeviceMapping {
    //   path_on_host: Some(String::from("/dev/ppp")),
    //   path_in_container: Some(String::from("/dev/ppp")),
    //   // cgroup_permissions: Strin,
    //   cgroup_permissions: Some(String::from("rwm")),
    // }]),
    ..Default::default()
  }
}

async fn create_ipsec_container(
  name: &str,
  config: &DaemonConfig,
  docker_api: &Docker,
) -> Result<(), DockerError> {
  let image = Some("hwdsl2/ipsec-vpn-server");
  let env = Some(vec![
    // "VPN_DNS_SRV1=127.0.0.1",
    // "VPN_DNS_SRV2=8.8.8.8"
  ]);
  let labels = Some(gen_labels_with_namespace("nanocl"));
  let host_config = Some(gen_ipsec_host_conf(config));
  let options = Some(CreateContainerOptions { name });
  let config = Config {
    image,
    env,
    labels,
    host_config,
    hostname: Some(name),
    domainname: Some(name),
    ..Default::default()
  };
  docker_api.create_container(options, config).await?;

  Ok(())
}

pub async fn boot(
  config: &DaemonConfig,
  docker_api: &Docker,
) -> Result<(), DockerError> {
  let container_name = "nanocl-vpn-ipsec";
  let s_state = get_service_state(container_name, docker_api).await;

  if s_state == ServiceState::Uninstalled {
    create_ipsec_container(container_name, config, docker_api).await?;
  }
  if s_state != ServiceState::Running {
    if let Err(err) = start_service(container_name, docker_api).await {
      log::error!("error while starting {} {}", container_name, err);
    }
  }

  Ok(())
}
