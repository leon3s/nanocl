use super::cli::Cli;

#[derive(Debug, Clone)]
pub struct DaemonConfig {
  pub(crate) hosts: Vec<String>,
  pub(crate) state_dir: String,
  // Todo use a config to setup deamon config
  #[allow(dead_code)]
  pub(crate) config_dir: String,
}

impl From<Cli> for DaemonConfig {
  fn from(args: Cli) -> Self {
    DaemonConfig {
      hosts: args.hosts,
      state_dir: args.state_dir,
      config_dir: args.config_dir,
    }
  }
}
