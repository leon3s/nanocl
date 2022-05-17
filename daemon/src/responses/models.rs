use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
  pub(crate) message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResponse {
  pub(crate) id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteResponse {
  pub(crate) count: u64,
}
