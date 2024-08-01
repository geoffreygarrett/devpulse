#[cfg(feature = "mongodb")]
use mongodb::{bson::Document, Client, error::Error as MongoError, options::ClientOptions};

#[cfg(feature = "mongodb")]
pub struct MongoDbPool;

#[cfg(feature = "mongodb")]
#[async_trait::async_trait]
impl DatabasePool for MongoDbPool {
    type Pool = Client;
    type FilterQueryType = Document; // MongoDB queries will be represented as BSON documents

    async fn new_pool(database_url: &str) -> Result<Self::Pool, Box<dyn std::error::Error>> {
        let client_options = ClientOptions::parse(database_url).await?;
        let client = Client::with_options(client_options)?;
        Ok(client)
    }
}
