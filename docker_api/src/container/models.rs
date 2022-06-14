#![allow(unused_imports, unused_qualifications, unused_extern_crates)]

#[cfg(feature = "utoipa")]
use utoipa::Component;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use serde::de::{DeserializeOwned, Deserializer};

use std::cmp::Eq;
use std::collections::HashMap;
use std::default::Default;
use std::hash::Hash;

use chrono::DateTime;
use chrono::Utc;

use crate::json_helper::deserialize_nonoptional_vec;

/// Configuration for a container that is portable between hosts.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ContainerConfig {
  /// The hostname to use for the container, as a valid RFC 1123 hostname.
  #[serde(rename = "Hostname")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub hostname: Option<String>,

  /// The domain name to use for the container.
  #[serde(rename = "Domainname")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub domainname: Option<String>,

  /// The user that commands are run as inside the container.
  #[serde(rename = "User")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user: Option<String>,

  /// Whether to attach to `stdin`.
  #[serde(rename = "AttachStdin")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub attach_stdin: Option<bool>,

  /// Whether to attach to `stdout`.
  #[serde(rename = "AttachStdout")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub attach_stdout: Option<bool>,

  /// Whether to attach to `stderr`.
  #[serde(rename = "AttachStderr")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub attach_stderr: Option<bool>,

  /// An object mapping ports to an empty object in the form:  `{\"<port>/<tcp|udp|sctp>\": {}}`
  #[serde(rename = "ExposedPorts")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub exposed_ports: Option<HashMap<String, HashMap<(), ()>>>,

  /// Attach standard streams to a TTY, including `stdin` if it is not closed.
  #[serde(rename = "Tty")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tty: Option<bool>,

  /// Open `stdin`
  #[serde(rename = "OpenStdin")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub open_stdin: Option<bool>,

  /// Close `stdin` after one attached client disconnects
  #[serde(rename = "StdinOnce")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stdin_once: Option<bool>,

  /// A list of environment variables to set inside the container in the form `[\"VAR=value\", ...]`. A variable without `=` is removed from the environment, rather than to have an empty value.
  #[serde(rename = "Env")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub env: Option<Vec<String>>,

  /// Command to run specified as a string or an array of strings.
  #[serde(rename = "Cmd")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cmd: Option<Vec<String>>,

  #[serde(rename = "Healthcheck")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub healthcheck: Option<HealthConfig>,

  /// Command is already escaped (Windows only)
  #[serde(rename = "ArgsEscaped")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub args_escaped: Option<bool>,

  /// The name (or reference) of the image to use when creating the container, or which was used when the container was created.
  #[serde(rename = "Image")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub image: Option<String>,

  /// An object mapping mount point paths inside the container to empty objects.
  #[serde(rename = "Volumes")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub volumes: Option<HashMap<String, HashMap<(), ()>>>,

  /// The working directory for commands to run in.
  #[serde(rename = "WorkingDir")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub working_dir: Option<String>,

  /// The entry point for the container as a string or an array of strings.  If the array consists of exactly one empty string (`[\"\"]`) then the entry point is reset to system default (i.e., the entry point used by docker when there is no `ENTRYPOINT` instruction in the `Dockerfile`).
  #[serde(rename = "Entrypoint")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub entrypoint: Option<Vec<String>>,

  /// Disable networking for the container.
  #[serde(rename = "NetworkDisabled")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub network_disabled: Option<bool>,

  /// MAC address of the container.
  #[serde(rename = "MacAddress")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mac_address: Option<String>,

  /// `ONBUILD` metadata that were defined in the image's `Dockerfile`.
  #[serde(rename = "OnBuild")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub on_build: Option<Vec<String>>,

  /// User-defined key/value metadata.
  #[serde(rename = "Labels")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub labels: Option<HashMap<String, String>>,

  /// Signal to stop a container as a string or unsigned integer.
  #[serde(rename = "StopSignal")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stop_signal: Option<String>,

  /// Timeout to stop a container in seconds.
  #[serde(rename = "StopTimeout")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stop_timeout: Option<i64>,

  /// Shell for when `RUN`, `CMD`, and `ENTRYPOINT` uses a shell.
  #[serde(rename = "Shell")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub shell: Option<Vec<String>>,
}

/// Parameters used in the [Create Container API](Docker::create_container())
///
/// ## Examples
///
/// ```rust
/// use docker_api::models::CreateContainerOptions;
///
/// let option = CreateContainerOptions {
///     name: String::from("my-super-container"),
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct CreateContainerOptions<T>
where
  T: Into<String> + Serialize,
{
  /// Assign the specified name to the container.
  pub name: T,
}

/// OK response to ContainerCreate operation
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ContainerCreateResponse {
  /// The ID of the created container
  #[serde(rename = "Id")]
  pub id: String,

  /// Warnings encountered when creating the container
  #[serde(rename = "Warnings")]
  #[serde(deserialize_with = "deserialize_nonoptional_vec")]
  pub warnings: Vec<String>,
}

/// Parameters used in the [Remove Container API](Docker::remove_container())
///
/// ## Examples
///
/// ```rust
/// use docker_api::models::RemoveContainerOptions;
///
/// use std::default::Default;
///
/// RemoveContainerOptions {
///     force: true,
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Copy, Clone, Default, PartialEq, Serialize)]
pub struct RemoveContainerOptions {
  /// Remove the volumes associated with the container.
  pub v: bool,
  /// If the container is running, kill it before removing it.
  pub force: bool,
  /// Remove the specified link associated with the container.
  pub link: bool,
}

/// Parameters used in the [Stop Container API](Docker::stop_container())
///
/// ## Examples
///
/// use docker_api::model::StopContainerOptions;
///
/// StopContainerOptions{
///     t: 30,
/// };
#[derive(Debug, Copy, Clone, Default, PartialEq, Serialize)]
pub struct StopContainerOptions {
  /// Number of seconds to wait before killing the container
  pub t: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ContainerSummary {
  /// The ID of this container
  #[serde(rename = "Id")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<String>,

  /// The names that this container has been given
  #[serde(rename = "Names")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub names: Option<Vec<String>>,

  /// The name of the image used when creating this container
  #[serde(rename = "Image")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub image: Option<String>,

  /// The ID of the image that this container was created from
  #[serde(rename = "ImageID")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub image_id: Option<String>,

  /// Command to run when starting the container
  #[serde(rename = "Command")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub command: Option<String>,

  /// When the container was created
  #[serde(rename = "Created")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created: Option<i64>,

  /// The ports exposed by this container
  #[serde(rename = "Ports")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ports: Option<Vec<Port>>,

  /// The size of files that have been created or changed by this container
  #[serde(rename = "SizeRw")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub size_rw: Option<i64>,

  /// The total size of all the files in this container
  #[serde(rename = "SizeRootFs")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub size_root_fs: Option<i64>,

  /// User-defined key/value metadata.
  #[serde(rename = "Labels")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub labels: Option<HashMap<String, String>>,

  /// The state of this container (e.g. `Exited`)
  #[serde(rename = "State")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub state: Option<String>,

  /// Additional human-readable status of this container (e.g. `Exit 0`)
  #[serde(rename = "Status")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,

  #[serde(rename = "HostConfig")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub host_config: Option<ContainerSummaryHostConfig>,

  #[serde(rename = "NetworkSettings")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub network_settings: Option<ContainerSummaryNetworkSettings>,

  #[serde(rename = "Mounts")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mounts: Option<Vec<MountPoint>>,
}

#[allow(non_camel_case_types)]
#[derive(
  Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord,
)]
pub enum MountPointTypeEnum {
  #[serde(rename = "")]
  Empty,
  #[serde(rename = "bind")]
  Bind,
  #[serde(rename = "volume")]
  Volume,
  #[serde(rename = "tmpfs")]
  Tmpfs,
  #[serde(rename = "npipe")]
  Npipe,
}

/// MountPoint represents a mount point configuration inside the container. This is used for reporting the mountpoints in use by a container.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MountPoint {
  /// The mount type:  - `bind` a mount of a file or directory from the host into the container. - `volume` a docker volume with the given `Name`. - `tmpfs` a `tmpfs`. - `npipe` a named pipe from the host into the container.
  #[serde(rename = "Type")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub typ: Option<MountPointTypeEnum>,

  /// Name is the name reference to the underlying data defined by `Source` e.g., the volume name.
  #[serde(rename = "Name")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,

  /// Source location of the mount.  For volumes, this contains the storage location of the volume (within `/var/lib/docker/volumes/`). For bind-mounts, and `npipe`, this contains the source (host) part of the bind-mount. For `tmpfs` mount points, this field is empty.
  #[serde(rename = "Source")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub source: Option<String>,

  /// Destination is the path relative to the container root (`/`) where the `Source` is mounted inside the container.
  #[serde(rename = "Destination")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub destination: Option<String>,

  /// Driver is the volume driver used to create the volume (if it is a volume).
  #[serde(rename = "Driver")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub driver: Option<String>,

  /// Mode is a comma separated list of options supplied by the user when creating the bind/volume mount.  The default is platform-specific (`\"z\"` on Linux, empty on Windows).
  #[serde(rename = "Mode")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mode: Option<String>,

  /// Whether the mount is mounted writable (read-write).
  #[serde(rename = "RW")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub rw: Option<bool>,

  /// Propagation describes how mounts are propagated from the host into the mount point, and vice-versa. Refer to the [Linux kernel documentation](https://www.kernel.org/doc/Documentation/filesystems/sharedsubtree.txt) for details. This field is not used on Windows.
  #[serde(rename = "Propagation")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub propagation: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ContainerSummaryHostConfig {
  #[serde(rename = "NetworkMode")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub network_mode: Option<String>,
}

/// A summary of the container's network settings
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ContainerSummaryNetworkSettings {
  #[serde(rename = "Networks")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub networks: Option<HashMap<String, EndpointSettings>>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartContainerOptions<T>
where
  T: Into<String> + Serialize,
{
  /// Override the key sequence for detaching a container. Format is a single character `[a-Z]` or
  /// `ctrl-<value>` where `<value>` is one of: `a-z`, `@`, `^`, `[`, `,` or `_`.
  pub detach_keys: T,
}

/// An open port on a container
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Port {
  /// Host IP address that the container's port is mapped to
  #[serde(rename = "IP")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ip: Option<String>,

  /// Port on the container
  #[serde(rename = "PrivatePort")]
  pub private_port: i64,

  /// Port exposed on the host
  #[serde(rename = "PublicPort")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub public_port: Option<i64>,

  #[serde(rename = "Type")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(with = "serde_with::rust::string_empty_as_none")]
  pub typ: Option<PortTypeEnum>,
}

#[allow(non_camel_case_types)]
#[derive(
  Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord,
)]
pub enum PortTypeEnum {
  #[serde(rename = "")]
  Empty,
  #[serde(rename = "tcp")]
  Tcp,
  #[serde(rename = "udp")]
  Udp,
  #[serde(rename = "sctp")]
  Sctp,
}

impl ::std::fmt::Display for PortTypeEnum {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    match *self {
      PortTypeEnum::Empty => write!(f, ""),
      PortTypeEnum::Tcp => write!(f, "tcp"),
      PortTypeEnum::Udp => write!(f, "udp"),
      PortTypeEnum::Sctp => write!(f, "sctp"),
    }
  }
}

impl ::std::str::FromStr for PortTypeEnum {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "" => Ok(PortTypeEnum::Empty),
      "tcp" => Ok(PortTypeEnum::Tcp),
      "udp" => Ok(PortTypeEnum::Udp),
      "sctp" => Ok(PortTypeEnum::Sctp),
      x => Err(format!("Invalid enum type: {}", x)),
    }
  }
}

impl ::std::convert::AsRef<str> for PortTypeEnum {
  fn as_ref(&self) -> &str {
    match self {
      PortTypeEnum::Empty => "",
      PortTypeEnum::Tcp => "tcp",
      PortTypeEnum::Udp => "udp",
      PortTypeEnum::Sctp => "sctp",
    }
  }
}

/// Configuration for a network endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EndpointSettings {
  #[serde(rename = "IPAMConfig")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ipam_config: Option<EndpointIpamConfig>,

  #[serde(rename = "Links")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub links: Option<Vec<String>>,

  #[serde(rename = "Aliases")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub aliases: Option<Vec<String>>,

  /// Unique ID of the network.
  #[serde(rename = "NetworkID")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub network_id: Option<String>,

  /// Unique ID for the service endpoint in a Sandbox.
  #[serde(rename = "EndpointID")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub endpoint_id: Option<String>,

  /// Gateway address for this network.
  #[serde(rename = "Gateway")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub gateway: Option<String>,

  /// IPv4 address.
  #[serde(rename = "IPAddress")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ip_address: Option<String>,

  /// Mask length of the IPv4 address.
  #[serde(rename = "IPPrefixLen")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ip_prefix_len: Option<i64>,

  /// IPv6 gateway address.
  #[serde(rename = "IPv6Gateway")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ipv6_gateway: Option<String>,

  /// Global IPv6 address.
  #[serde(rename = "GlobalIPv6Address")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub global_ipv6_address: Option<String>,

  /// Mask length of the global IPv6 address.
  #[serde(rename = "GlobalIPv6PrefixLen")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub global_ipv6_prefix_len: Option<i64>,

  /// MAC address for the endpoint on this network.
  #[serde(rename = "MacAddress")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mac_address: Option<String>,

  /// DriverOpts is a mapping of driver options and values. These options are passed directly to the driver and are driver specific.
  #[serde(rename = "DriverOpts")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub driver_opts: Option<HashMap<String, String>>,
}

/// EndpointIPAMConfig represents an endpoint's IPAM configuration.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EndpointIpamConfig {
  #[serde(rename = "IPv4Address")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ipv4_address: Option<String>,

  #[serde(rename = "IPv6Address")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ipv6_address: Option<String>,

  #[serde(rename = "LinkLocalIPs")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub link_local_i_ps: Option<Vec<String>>,
}

/// A test to perform to check that the container is healthy.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HealthConfig {
  /// The test to perform. Possible values are:  - `[]` inherit healthcheck from image or parent image - `[\"NONE\"]` disable healthcheck - `[\"CMD\", args...]` exec arguments directly - `[\"CMD-SHELL\", command]` run command with system's default shell
  #[serde(rename = "Test")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub test: Option<Vec<String>>,

  /// The time to wait between checks in nanoseconds. It should be 0 or at least 1000000 (1 ms). 0 means inherit.
  #[serde(rename = "Interval")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub interval: Option<i64>,

  /// The time to wait before considering the check to have hung. It should be 0 or at least 1000000 (1 ms). 0 means inherit.
  #[serde(rename = "Timeout")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub timeout: Option<i64>,

  /// The number of consecutive failures needed to consider a container as unhealthy. 0 means inherit.
  #[serde(rename = "Retries")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub retries: Option<i64>,

  /// Start period for the container to initialize before starting health-retries countdown in nanoseconds. It should be 0 or at least 1000000 (1 ms). 0 means inherit.
  #[serde(rename = "StartPeriod")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub start_period: Option<i64>,
}

#[cfg(test)]
mod test_models {
  use super::*;

  #[test]
  fn test_init_stop_container_options() {
    let _item = StopContainerOptions::default();
  }

  #[test]
  fn test_init_remove_container_options() {
    let _item = RemoveContainerOptions::default();
  }
}
