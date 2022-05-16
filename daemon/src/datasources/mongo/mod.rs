use std::fmt::Debug;
use mongodb::{Client, options::ClientOptions, Database};

pub mod models;
pub mod repository;

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
}

pub async fn connect() -> Result<DatasourceMongoDb, mongodb::error::Error> {
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

#[cfg(test)]
mod mongodb_tests {
  use super::{connect, models};

  #[ntex::test]
  async fn test_connect() -> Result<(), mongodb::error::Error> {
    let datasource = connect().await?;
    assert!(datasource.db.name() == "nanocldb");
    Ok(())
  }

  #[ntex::test]
  async fn test_repository() -> Result<(), mongodb::error::Error> {
    let datasource = connect().await?;
    let repository = datasource.new_repository::<models::Namespace>("namespace");
    repository.list().await?;
    let new_namespace = models::Namespace {
      name: String::from("new_entry"),
      ..models::Namespace::default()
    };
    let id = repository.create(new_namespace).await?;
    println!("id: {}", id);
    Ok(())
  }
}
