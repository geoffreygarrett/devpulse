#[cfg(feature = "postgres")]
use sqlx::PgPool;

use crate::adapters::traits::database_pool::DatabasePool;

#[cfg(feature = "postgres")]
pub struct PostgresDbPool;

#[cfg(feature = "postgres")]
#[async_trait::async_trait]
impl DatabasePool for PostgresDbPool {
    type Pool = PgPool;

    type QueryType = String;

    async fn new_pool(database_url: &str) -> Result<Self::Pool, Box<dyn std::error::Error>> {
        let pool = PgPool::connect(database_url).await?;
        Ok(pool)
    }
}
