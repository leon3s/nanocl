use std::fmt::Debug;
use mongodb::{Client, options::ClientOptions, Database};

use super::repository;

#[derive(Debug, Clone)]
pub struct DatasourceMongoDb {
  pub(crate) db: Database,
}

impl DatasourceMongoDb {
  pub(crate) fn new_repository<T>(&self, name: &'static str) -> repository::Repository<T> {
    let collection = self.db.collection::<T>(name);
    repository::Repository {
      collection,
    }
  }

  pub(crate) async fn connect() -> Result<DatasourceMongoDb, mongodb::error::Error> {
    // Parse a connection string into an options struct.
    let client_options = ClientOptions::parse("mongodb://root:root@localhost:27017/").await?;
    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;
    let db = client.database("nanocldb");
    let datasource = DatasourceMongoDb {
      db,
    };
    Ok(datasource)
  }
}

#[cfg(test)]
mod mongodb_tests {
  use mongodb::bson::doc;
  
  use super::DatasourceMongoDb;
  use crate::models::namespace::Namespace;

  #[ntex::test]
  async fn test_connect() -> Result<(), mongodb::error::Error> {
    let datasource = DatasourceMongoDb::connect().await?;
    assert!(datasource.db.name() == "nanocldb");
    Ok(())
  }

  #[ntex::test]
  async fn test_repository() -> Result<(), mongodb::error::Error> {
    // Connect datasource to mongodb
    let datasource = DatasourceMongoDb::connect().await?;
    // Generate a repository
    let repository = datasource.new_repository::<Namespace>("namespace");
    // Delete Everything before test
    repository.delete(doc! {}).await?;
    // Expect to find 0 elements
    let find_resp = repository.find().await?;
    assert_eq!(find_resp.len(), 0);
    // Create a new namespace //
    let new_namespace = Namespace {
      name: String::from("new_entry"),
      ..Namespace::default()
    };
    repository.create(new_namespace).await?;
    // Expect to find 1 elements
    let find_resp = repository.find().await?;
    assert_eq!(find_resp.len(), 1);
    let id_to_delete = find_resp[0].id.to_string();
    // Expect to delete one element
    let delete_resp = repository.delete_by_id(id_to_delete).await?;
    assert_eq!(delete_resp, 1);
    Ok(())
  }
}
