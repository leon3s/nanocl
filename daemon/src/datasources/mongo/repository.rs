use mongodb::Collection;
use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug, Clone)]
pub struct Repository<T> {
  pub(crate) collection: Collection<T>,
}

impl<T> Repository<T> {
  pub async fn list(&self) -> Result<Vec<T>, mongodb::error::Error> where
  T: DeserializeOwned {
    let mut items: Vec<T> = Vec::new();
    let mut cursor = self.collection.find(None, None).await?;
    while cursor.advance().await? {
      let item = cursor.deserialize_current()?;
      items.push(item);
    }
    Ok(items)
  }

  pub async fn create(&self, item: T) -> Result<String, mongodb::error::Error>
  where
    T: Serialize {
    let result = self.collection.insert_one(item, None).await?;
    Ok(result.inserted_id.to_string())
  }
}
