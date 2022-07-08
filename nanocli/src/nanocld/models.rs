use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PgGenericDelete {
  pub(crate) count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PgGenericCount {
  pub(crate) count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenericNamespaceQuery {
  pub(crate) namespace: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProgressDetail {
  #[serde(rename = "current")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub current: Option<i64>,

  #[serde(rename = "total")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub total: Option<i64>,
}
