use std::hash::Hash;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

/// Image ID or Digest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ImageId {
  #[serde(rename = "ID")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<String>,
}

/// Parameters to the [Build Image API](Docker::build_image())
///
/// ## Examples
///
/// ```rust
/// use docker_api::image::models::BuildImageOptions;
///
/// BuildImageOptions {
///     dockerfile: "Dockerfile",
///     t: "my-image",
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct BuildImageOptions<T>
where
  T: Into<String> + Eq + Hash + Serialize,
{
  /// Path within the build context to the `Dockerfile`. This is ignored if `remote` is specified and
  /// points to an external `Dockerfile`.
  pub dockerfile: T,
  /// A name and optional tag to apply to the image in the `name:tag` format. If you omit the tag
  /// the default `latest` value is assumed. You can provide several `t` parameters.
  pub t: T,
  /// Extra hosts to add to `/etc/hosts`.
  pub extrahosts: Option<T>,
  /// A Git repository URI or HTTP/HTTPS context URI. If the URI points to a single text file,
  /// the file’s contents are placed into a file called `Dockerfile` and the image is built from
  /// that file. If the URI points to a tarball, the file is downloaded by the daemon and the
  /// contents therein used as the context for the build. If the URI points to a tarball and the
  /// `dockerfile` parameter is also specified, there must be a file with the corresponding path
  /// inside the tarball.
  pub remote: T,
  /// Suppress verbose build output.
  pub q: bool,
  /// Do not use the cache when building the image.
  pub nocache: bool,
  /// JSON array of images used for build cache resolution.
  #[serde(serialize_with = "crate::json_helper::serialize_as_json")]
  pub cachefrom: Vec<T>,
  /// Attempt to pull the image even if an older image exists locally.
  pub pull: bool,
  /// Remove intermediate containers after a successful build.
  pub rm: bool,
  /// Always remove intermediate containers, even upon failure.
  pub forcerm: bool,
  /// Set memory limit for build.
  pub memory: Option<u64>,
  /// Total memory (memory + swap). Set as `-1` to disable swap.
  pub memswap: Option<i64>,
  /// CPU shares (relative weight).
  pub cpushares: Option<u64>,
  /// CPUs in which to allow execution (e.g., `0-3`, `0,1`).
  pub cpusetcpus: T,
  /// The length of a CPU period in microseconds.
  pub cpuperiod: Option<u64>,
  /// Microseconds of CPU time that the container can get in a CPU period.
  pub cpuquota: Option<u64>,
  /// JSON map of string pairs for build-time variables. Users pass these values at build-time.
  /// Docker uses the buildargs as the environment context for commands run via the `Dockerfile`
  /// RUN instruction, or for variable expansion in other `Dockerfile` instructions.
  #[serde(serialize_with = "crate::json_helper::serialize_as_json")]
  pub buildargs: HashMap<T, T>,
  /// Size of `/dev/shm` in bytes. The size must be greater than 0. If omitted the system uses 64MB.
  pub shmsize: Option<u64>,
  /// Squash the resulting images layers into a single layer.
  pub squash: bool,
  /// Arbitrary key/value labels to set on the image, as a JSON map of string pairs.
  #[serde(serialize_with = "crate::json_helper::serialize_as_json")]
  pub labels: HashMap<T, T>,
  /// Sets the networking mode for the run commands during build. Supported standard values are:
  /// `bridge`, `host`, `none`, and `container:<name|id>`. Any other value is taken as a custom network's
  /// name to which this container should connect to.
  pub networkmode: T,
  /// Platform in the format `os[/arch[/variant]]`
  pub platform: T,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BuildInfo {
  #[serde(rename = "id")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<String>,

  #[serde(rename = "stream")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stream: Option<String>,

  #[serde(rename = "error")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub error: Option<String>,

  #[serde(rename = "errorDetail")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub error_detail: Option<ErrorDetail>,

  #[serde(rename = "status")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,

  #[serde(rename = "progress")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub progress: Option<String>,

  #[serde(rename = "progressDetail")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub progress_detail: Option<ProgressDetail>,

  #[serde(rename = "aux")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub aux: Option<ImageId>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
  #[serde(rename = "code")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub code: Option<i64>,

  #[serde(rename = "message")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProgressDetail {
  #[serde(rename = "current")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub current: Option<i64>,

  #[serde(rename = "total")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub total: Option<i64>,
}
