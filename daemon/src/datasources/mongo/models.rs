use serde::{Deserialize, Serialize};
use mongodb::bson::{oid::ObjectId, serde_helpers::serialize_object_id_as_hex_string};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Namespace {
  #[serde(rename = "_id", serialize_with = "serialize_object_id_as_hex_string")]
  pub id: ObjectId,
  pub name: String,
}

impl Default for Namespace {
    fn default() -> Self {
        Self {
          id: ObjectId::new(),
          name: Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct NamespaceRelation {
  pub namespace_id: ObjectId,
  pub mutex_id: ObjectId,
  pub mutex_type: String,
}
