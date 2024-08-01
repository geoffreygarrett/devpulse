use async_trait::async_trait;

#[async_trait]
pub trait DatabasePool {
    type Pool;
    type FilerQueryType;

    async fn new_pool(database_url: &str) -> Result<Self::Pool, Box<dyn std::error::Error>>;
}

pub trait FilterToQuery<DB: DatabasePool> {
    fn to_query(&self) -> DB::FilerQueryType;
}
