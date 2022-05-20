use uuid::Uuid;
use diesel::prelude::*;

use crate::models::{NamespaceItem, NamespaceCreate};

pub fn create(
  item: NamespaceCreate,
  conn: &PgConnection,
) -> Result<NamespaceItem, diesel::result::Error>{
  use crate::schema::namespaces::dsl::namespaces;
  let new_namespace = NamespaceItem {
    id: Uuid::new_v4(),
    name: item.name,
  };
  diesel::insert_into(namespaces)
  .values(&new_namespace)
  .execute(conn)?;
  Ok(new_namespace)
}

pub fn find_by_name() {
}

pub fn delete_by_name() {
}
