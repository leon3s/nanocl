use thiserror::Error;

use crate::nanocld::error::NanocldError;

#[derive(Debug, Error)]
pub enum CliError {
  #[error("io error")]
  Io(#[from] std::io::Error),
  #[error("yaml parse error")]
  Parse(#[from] serde_yaml::Error),
  #[error("got client error")]
  Client(#[from] NanocldError),
}
