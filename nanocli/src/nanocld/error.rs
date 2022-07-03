use thiserror::Error;
use serde::{Serialize, Deserialize};
use ntex::http::{
  StatusCode,
  error::PayloadError,
  client::{
    ClientResponse,
    error::{SendRequestError, JsonPayloadError},
  },
};

#[derive(Debug, Error, Serialize, Deserialize)]
pub struct ApiError {
  pub msg: String,
}

impl std::fmt::Display for ApiError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", &self.msg)
  }
}

#[derive(Debug, Error)]
pub enum NanocldError {
  #[error("received daemon error")]
  Api(#[from] ApiError),
  #[error("got payload error")]
  Payload(#[from] PayloadError),
  #[error("send request error")]
  SendRequest(#[from] SendRequestError),
  #[error("json parse error")]
  JsonPayload(#[from] JsonPayloadError),
}

pub async fn is_api_error(
  res: &mut ClientResponse,
  status: &StatusCode,
) -> Result<(), NanocldError> {
  if status.is_server_error() || status.is_client_error() {
    let err = res.json::<ApiError>().await?;
    return Err(NanocldError::Api(err));
  }
  Ok(())
}
