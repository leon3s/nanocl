use clap::{AppSettings, Parser};
/// nanocl daemon
/// self-sufficient intranet
#[derive(Debug, Parser)]
#[clap(
  about,
  version,
  global_setting = AppSettings::DeriveDisplayOrder,
)]
pub(crate) struct Cli {
  /// Boot only then stop. Used to just download required components
  #[clap(long)]
  pub(crate) boot_only: bool,
  /// Daemon socket(s) to connect to default to unix:///run/nanocl/nanocl.sock
  #[clap(short, long = "--host")]
  pub(crate) host: Option<String>,
}
