use clap::{AppSettings, Parser};

/// nanocl daemon
/// vms and containers manager at scale for intranet
#[derive(Debug, Clone, Parser)]
#[clap(
  about,
  version,
  global_setting = AppSettings::DeriveDisplayOrder,
)]
pub(crate) struct Cli {
  #[clap(long)]
  pub(crate) genopenapi: bool,
  /// Only instally required services this have to be called before any boot
  #[clap(long)]
  pub(crate) install_services: bool,
  /// Daemon socket to connect to default to unix:///run/nanocl/nanocl.sock
  #[clap(
    short = 'H',
    long = "--host",
    default_value = "unix:///run/nanocl/nanocl.sock"
  )]
  pub(crate) hosts: Vec<String>,
  /// Docker daemon socket to connect to
  #[clap(long, default_value = "unix:///run/docker.sock")]
  pub(crate) docker_host: String,
  /// Nanocld state dir
  #[clap(long, default_value = "/var/lib/nanocl")]
  pub(crate) state_dir: String,
  /// Nanocld config dir
  #[clap(long, default_value = "/etc/nanocl")]
  pub(crate) config_dir: String,
}
