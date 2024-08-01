use crate::adapters::traits::database_pool::DatabasePool;

pub trait FilterToQuery<DB: DatabasePool> {
    type QueryType;
    fn to_query(&self) -> DB::QueryType;
}