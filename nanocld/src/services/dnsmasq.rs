use regex::Regex;
use ntex::http::StatusCode;
use std::{
  io::Write,
  path::{Path, PathBuf},
  fs,
  collections::HashMap,
};

use bollard::{
  Docker,
  models::{HostConfig, PortBinding},
  errors::Error as DockerError,
  container::{Config, CreateContainerOptions},
};

use thiserror::Error;
use regex::Error as RegexError;
use std::io::Error as IoError;

use crate::config::DaemonConfig;
use crate::errors::{HttpResponseError, IntoHttpResponseError};

use super::utils::*;

use crate::services::errors::docker_error_ref;

#[derive(Debug, Error)]
pub enum DnsmasqError {
  #[error("dnsmasq io error")]
  Io(#[from] IoError),
  #[error("dnsmasq regex error")]
  Regex(#[from] RegexError),
  #[error("dnsmasq docker_api error")]
  Docker(#[from] DockerError),
}

impl IntoHttpResponseError for DnsmasqError {
  fn to_http_error(&self) -> HttpResponseError {
    match self {
      DnsmasqError::Io(err) => HttpResponseError {
        msg: format!("dnsmasq io error {:#?}", err),
        status: StatusCode::INTERNAL_SERVER_ERROR,
      },
      DnsmasqError::Regex(err) => HttpResponseError {
        msg: format!("dnsmasq regex error {:#?}", err),
        status: StatusCode::INTERNAL_SERVER_ERROR,
      },
      DnsmasqError::Docker(err) => docker_error_ref(err),
    }
  }
}

fn write_dns_entry_conf(path: &PathBuf, content: &str) -> std::io::Result<()> {
  let mut f = fs::File::create(path)?;
  f.write_all(content.as_bytes())?;
  f.sync_data()?;
  Ok(())
}

/// # Add or Update a dns entry on dnsmasq
///
/// # Arguments
pub fn add_dns_entry(
  domain_name: &str,
  ip_address: &str,
  state_dir: &str,
) -> Result<(), DnsmasqError> {
  let file_path = Path::new(state_dir).join("dnsmasq/dnsmasq.d/dns_entry.conf");
  let content = fs::read_to_string(&file_path)?;
  let reg_expr = r"address=/.".to_owned() + domain_name + "/.*";

  let reg = Regex::new(&reg_expr)?;

  let new_dns_entry = "address=/.".to_owned() + domain_name + "/" + ip_address;
  if reg.is_match(&content) {
    // If entry exist we just update it by replacing it with the regex
    let res = reg.replace_all(&content, &new_dns_entry);
    let new_content = res.to_string();
    write_dns_entry_conf(&file_path, &new_content)?;
  } else {
    // else we just add it at end of file.
    let mut file = fs::OpenOptions::new()
      .write(true)
      .append(true)
      .open(file_path)?;

    writeln!(file, "{}", &new_dns_entry)?;
  }

  Ok(())
}

pub async fn restart(docker_api: &Docker) -> Result<(), DnsmasqError> {
  docker_api
    .restart_container("nanocl-dns-dnsmasq", None)
    .await?;
  Ok(())
}

pub fn gen_dnsmasq_host_conf(config: &DaemonConfig) -> HostConfig {
  let config_file_path =
    Path::new(&config.state_dir).join("dnsmasq/dnsmasq.conf");
  let dir_path = Path::new(&config.state_dir).join("dnsmasq/dnsmasq.d/");
  let binds = Some(vec![
    format!("{}:/etc/dnsmasq.conf", config_file_path.display()),
    format!("{}:/etc/dnsmasq.d/", dir_path.display()),
  ]);
  let mut port_bindings: HashMap<String, Option<Vec<PortBinding>>> =
    HashMap::new();
  port_bindings.insert(
    String::from("53/udp"),
    Some(vec![PortBinding {
      host_ip: None,
      host_port: Some(String::from("53/udp")),
    }]),
  );
  port_bindings.insert(
    String::from("53/tcp"),
    Some(vec![PortBinding {
      host_ip: None,
      host_port: Some(String::from("53/tcp")),
    }]),
  );
  HostConfig {
    binds,
    cap_add: Some(vec![String::from("NET_ADMIN")]),
    network_mode: Some(String::from("host")),
    // port_bindings: Some(port_bindings),
    ..Default::default()
  }
}

async fn create_dnsmasq_container(
  name: &str,
  config: &DaemonConfig,
  docker_api: &Docker,
) -> Result<(), DockerError> {
  let image = Some("nanocl-dns-dnsmasq:latest");
  let labels = Some(gen_labels_with_namespace("nanocl"));
  let host_config = Some(gen_dnsmasq_host_conf(config));
  let options = Some(CreateContainerOptions { name });
  let config = Config {
    image,
    labels,
    host_config,
    tty: Some(true),
    attach_stdout: Some(true),
    attach_stderr: Some(true),
    ..Default::default()
  };
  docker_api.create_container(options, config).await?;
  Ok(())
}

pub async fn boot(
  config: &DaemonConfig,
  docker_api: &Docker,
) -> Result<(), DockerError> {
  let container_name = "nanocl-dns-dnsmasq";
  let s_state = get_service_state(container_name, docker_api).await;

  if s_state == ServiceState::Uninstalled {
    create_dnsmasq_container(container_name, config, docker_api).await?;
  }
  if s_state != ServiceState::Running {
    if let Err(err) = start_service(container_name, docker_api).await {
      log::error!("error while starting {} {}", container_name, err);
    }
  }
  Ok(())
}

#[cfg(test)]
mod tests {

  use super::*;

  use crate::utils::test::*;

  struct TestDomain {
    name: String,
    ip_address: String,
  }

  #[ntex::test]
  async fn test_add_dns_entry() -> TestReturn {
    const STATE_DIR: &str = "/var/lib/nanocl";
    let file_path =
      Path::new(STATE_DIR).join("dnsmasq/dnsmasq.d/dns_entry.conf");
    let saved_content = fs::read_to_string(&file_path)?;
    write_dns_entry_conf(&file_path, "")?;
    let test_1 = TestDomain {
      name: String::from("test.com"),
      ip_address: String::from("141.0.0.1"),
    };
    let test_2 = TestDomain {
      name: String::from("test2.com"),
      ip_address: String::from("122.0.0.1"),
    };
    add_dns_entry(&test_1.name, &test_1.ip_address, STATE_DIR)?;
    add_dns_entry(&test_2.name, &test_2.ip_address, STATE_DIR)?;
    let content = fs::read_to_string(&file_path)?;
    let expected_content = format!(
      "address=/.{}/{}\naddress=/.{}/{}\n",
      &test_1.name, &test_1.ip_address, &test_2.name, &test_2.ip_address
    );
    assert_eq!(content, expected_content);
    let test_3 = TestDomain {
      ip_address: String::from("121.0.0.1"),
      ..test_2
    };
    add_dns_entry(&test_3.name, &test_3.ip_address, STATE_DIR)?;
    let content = fs::read_to_string(&file_path)?;
    let expected_content = format!(
      "address=/.{}/{}\naddress=/.{}/{}\n",
      &test_1.name, &test_1.ip_address, &test_3.name, &test_3.ip_address
    );
    assert_eq!(content, expected_content);
    write_dns_entry_conf(&file_path, &saved_content)?;
    Ok(())
  }
}
