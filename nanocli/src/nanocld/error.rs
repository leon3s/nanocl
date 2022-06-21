use serde::{Serialize, Deserialize};
use ntex::http::{
  client::error::{SendRequestError, JsonPayloadError},
  error::PayloadError,
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
