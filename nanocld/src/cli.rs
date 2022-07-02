use clap::{AppSettings, Parser};
/// A self-sufficient vms and containers manager
#[derive(Debug, Parser)]
#[clap(
  about,
  version,
  global_setting = AppSettings::DeriveDisplayOrder,
)]
pub(crate) struct Cli {
  /// commands
  #[clap(long)]
  pub(crate) boot_only: bool,
  // #[clap(subcommand)]
  // pub command: Commands,
}
