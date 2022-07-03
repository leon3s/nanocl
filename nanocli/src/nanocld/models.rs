use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PgGenericDelete {
  pub(crate) count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PgGenericCount {
  pub(crate) count: usize,
}
