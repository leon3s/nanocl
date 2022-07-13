use super::cli::Cli;

#[derive(Debug, Clone)]
pub struct DaemonConfig {
  pub(crate) host: String,
  pub(crate) state_dir: String,
  pub(crate) config_dir: String,
}

impl From<Cli> for DaemonConfig {
  fn from(args: Cli) -> Self {
    DaemonConfig {
      host: args.host,
      state_dir: args.state_dir,
      config_dir: args.config_dir,
    }
  }
}
