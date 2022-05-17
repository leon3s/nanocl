use mongodb::error::Error as MongoError;
use mongodb::{Collection, bson::{Document, doc}};
use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug, Clone)]
pub struct Repository<T> {
  pub(crate) collection: Collection<T>,
}

impl<T> Repository<T> {
  pub async fn find(&self) -> Result<Vec<T>, MongoError> where
  T: DeserializeOwned {
    let mut items: Vec<T> = Vec::new();
    let mut cursor = self.collection.find(None, None).await?;
    while cursor.advance().await? {
      let item = cursor.deserialize_current()?;
      items.push(item);
    }
    Ok(items)
  }

  pub async fn create(&self, item: T) -> Result<String, MongoError>
  where
    T: Serialize {
    let result = self
    .collection
    .insert_one(
      item,
      None
    ).await?
    .inserted_id
    .to_string();
    Ok(result)
  }

  pub async fn delete(&self, query: Document) -> Result<u64, MongoError> {
    let result = self.collection.delete_many(query, None).await?;
    Ok(result.deleted_count)
  }

  pub async fn delete_one(&self, query: Document) -> Result<u64, MongoError> {
    let result = self.collection.delete_one(
      query,
      None
    ).await?;
    Ok(
      result.deleted_count
    )
  }

  pub async fn delete_by_id(&self, id: String) -> Result<u64, MongoError> {
    println!("debug id {}", id);
    self.delete_one(doc! {
      "_id": id,
    }).await
  }
}
