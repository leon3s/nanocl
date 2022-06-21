use serde::{Serialize, Deserialize};
use ntex::http::{
  client::{
    error::{SendRequestError, JsonPayloadError},
    ClientResponse,
  },
  error::PayloadError,
  StatusCode,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
  pub msg: String,
}

#[derive(Debug)]
pub enum Error {
  Api(ApiError),
  Payload(PayloadError),
  SendRequest(SendRequestError),
  JsonPayload(JsonPayloadError),
}

pub async fn is_api_error(
  res: &mut ClientResponse,
  status: &StatusCode,
) -> Result<(), Error> {
  if status.is_server_error() || status.is_client_error() {
    let err = res.json::<ApiError>().await.map_err(Error::JsonPayload)?;
    return Err(Error::Api(err));
  }
  Ok(())
}
