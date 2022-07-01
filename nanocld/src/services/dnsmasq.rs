use std::{fs, collections::HashMap};
use std::fs::OpenOptions;
use std::io::Write;
use regex::Regex;
use ntex::http::StatusCode;

use bollard::{
  Docker,
  models::{HostConfig, PortBinding},
  errors::Error as DockerError,
  container::{Config, CreateContainerOptions},
};

use thiserror::Error;
use regex::Error as RegexError;
use std::io::Error as IoError;

use crate::controllers::errors::{HttpError, IntoHttpError};

use super::utils::*;

use crate::services::errors::docker_error_ref;

#[derive(Debug, Error)]
pub enum DnsmasqError {
  #[error("dnsmasq io error")]
  Io(#[from] IoError),
  #[error("dnsmasq regex error")]
  Regex(#[from] RegexError),
  #[error("dnsmasq docker error")]
  Docker(#[from] DockerError),
}

impl IntoHttpError for DnsmasqError {
  fn to_http_error(&self) -> HttpError {
    match self {
      DnsmasqError::Io(err) => HttpError {
        msg: format!("dnsmasq io error {:#?}", err),
        status: StatusCode::INTERNAL_SERVER_ERROR,
      },
      DnsmasqError::Regex(err) => HttpError {
        msg: format!("dnsmasq regex error {:#?}", err),
        status: StatusCode::INTERNAL_SERVER_ERROR,
      },
      DnsmasqError::Docker(err) => docker_error_ref(err),
    }
  }
}

fn write_dns_entry_conf(path: &str, content: &str) -> std::io::Result<()> {
  let mut f = fs::File::create(path)?;
  f.write_all(content.as_bytes())?;
  f.sync_data()?;
  Ok(())
}

/// Mostly presend for tests
fn read_dns_entry_conf(path: &str) -> std::io::Result<String> {
  fs::read_to_string(path)
}

/// # Add or Update a dns entry on dnsmasq
///
/// # Arguments
/// # TODO doc this
pub fn add_dns_entry(
  domain_name: &str,
  ip_address: &str,
) -> Result<(), DnsmasqError> {
  let path = "/var/lib/nanocl/dnsmasq/dnsmasq.d/dns_entry.conf";
  let content = read_dns_entry_conf(path).map_err(DnsmasqError::Io)?;
  let reg_expr = r"address=/".to_owned() + domain_name + "/.*";

  let reg = Regex::new(&reg_expr).map_err(DnsmasqError::Regex)?;

  let new_dns_entry = "address=/".to_owned() + domain_name + "/" + ip_address;
  if reg.is_match(&content) {
    // If entry exist we just update it by replacing it with the regex
    let res = reg.replace_all(&content, &new_dns_entry);
    let new_content = res.to_string();
    write_dns_entry_conf(path, &new_content).map_err(DnsmasqError::Io)?;
  } else {
    // else we just add it at end of file.
    let mut file = OpenOptions::new()
      .write(true)
      .append(true)
      .open(path)
      .map_err(DnsmasqError::Io)?;

    writeln!(file, "{}", &new_dns_entry).map_err(DnsmasqError::Io)?;
  }

  Ok(())
}

pub async fn restart(docker: &Docker) -> Result<(), DnsmasqError> {
  docker
    .restart_container("nanocl-dns-dnsmasq", None)
    .await
    .map_err(DnsmasqError::Docker)?;
  Ok(())
}

pub fn gen_dnsmasq_host_conf() -> HostConfig {
  let binds = Some(vec![
    String::from("/var/lib/nanocl/dnsmasq/dnsmasq.conf:/etc/dnsmasq.conf"),
    String::from("/var/lib/nanocl/dnsmasq/dnsmasq.d:/etc/dnsmasq.d"),
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
    port_bindings: Some(port_bindings),
    ..Default::default()
  }
}

async fn create_dnsmasq_container(
  docker: &Docker,
  name: &str,
) -> Result<(), DockerError> {
  let image = Some("nanocl-dns-dnsmasq:latest");
  let labels = Some(gen_labels_with_namespace("nanocl"));
  let host_config = Some(gen_dnsmasq_host_conf());
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
  docker.create_container(options, config).await?;
  Ok(())
}

pub async fn boot(docker: &Docker) -> Result<(), DockerError> {
  let container_name = "nanocl-dns-dnsmasq";
  build_service(docker, "nanocl-dns-dnsmasq").await?;
  let s_state = get_service_state(docker, container_name).await;

  if s_state == ServiceState::Uninstalled {
    create_dnsmasq_container(docker, container_name).await?;
  }
  if s_state != ServiceState::Running {
    if let Err(err) = start_service(docker, container_name).await {
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
    const CONF_PATH: &str = "/var/lib/nanocl/dnsmasq/dnsmasq.d/dns_entry.conf";
    let saved_content = read_dns_entry_conf(CONF_PATH)?;
    write_dns_entry_conf(CONF_PATH, "")?;
    let test_1 = TestDomain {
      name: String::from("test.com"),
      ip_address: String::from("141.0.0.1"),
    };
    let test_2 = TestDomain {
      name: String::from("test2.com"),
      ip_address: String::from("122.0.0.1"),
    };
    add_dns_entry(&test_1.name, &test_1.ip_address)?;
    add_dns_entry(&test_2.name, &test_2.ip_address)?;
    let content = read_dns_entry_conf(CONF_PATH)?;
    let expected_content = format!(
      "address=/{}/{}\naddress=/{}/{}\n",
      &test_1.name, &test_1.ip_address, &test_2.name, &test_2.ip_address
    );
    assert_eq!(content, expected_content);
    let test_3 = TestDomain {
      ip_address: String::from("121.0.0.1"),
      ..test_2
    };
    add_dns_entry(&test_3.name, &test_3.ip_address)?;
    let content = read_dns_entry_conf(CONF_PATH)?;
    let expected_content = format!(
      "address=/{}/{}\naddress=/{}/{}\n",
      &test_1.name, &test_1.ip_address, &test_3.name, &test_3.ip_address
    );
    assert_eq!(content, expected_content);
    write_dns_entry_conf(CONF_PATH, &saved_content)?;
    Ok(())
  }
}
