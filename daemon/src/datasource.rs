use mongodb::{Client, options::ClientOptions, Database, error::Error};

pub async fn mongo_connect() -> Result<Database, Error> {
    // Parse a connection string into an options struct.
  let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
  // Get a handle to the deployment.
  let client = Client::with_options(client_options)?;
  let db = client.database("nanocldb");
  Ok(db)
}
