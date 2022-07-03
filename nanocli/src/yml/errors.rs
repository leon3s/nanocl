use thiserror::Error;

use crate::nanocld::error::NanocldError;

#[derive(Debug, Error)]
pub enum YmlConfigError {
  #[error("io error")]
  Io(#[from] std::io::Error),
  #[error("yaml parse error")]
  Parse(#[from] serde_yaml::Error),
  #[error("nanocld client error")]
  Nanocld(#[from] NanocldError),
}
