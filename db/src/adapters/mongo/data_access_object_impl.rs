use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::Collection;

use crate::adapters::traits::database_pool::DatabasePool;
use crate::adapters::traits::filter::FilterToQuery;

pub struct MongoDao<Entity>
where
    Entity: serde::de::DeserializeOwned + serde::Serialize + Unpin + Send + Sync,
{
    collection: Collection<Entity>,
}

#[async_trait]
impl<Entity, InsertModel, UpdateModel>
    DataAccessObject<Entity, InsertModel, UpdateModel, MongoDbPool> for MongoDao<Entity>
where
    Entity: serde::de::DeserializeOwned + serde::Serialize + Unpin + Send + Sync,
    InsertModel: serde::Serialize + Send + Sync,
    UpdateModel: serde::Serialize + Send + Sync,
    FilterCondition: FilterToQuery<MongoDbPool, QueryType = mongodb::bson::Document>,
{
    type Error = mongodb::error::Error;
    type QueryParams = FilterCondition;

    async fn insert(
        &self, item: &InsertModel, _params: Option<Self::QueryParams>,
    ) -> Result<Option<Entity>, Self::Error> {
        let insert_result = self.collection.insert_one(item, None).await?;
        if insert_result.acknowledged {
            Ok(Some(
                self.collection
                    .find_one(doc! { "_id": insert_result.inserted_id }, None)
                    .await?,
            ))
        } else {
            Ok(None)
        }
    }

    async fn upsert(
        &self, item: &InsertModel, _conflict_resolution: &str, _params: Option<Self::QueryParams>,
    ) -> Result<Option<Entity>, Self::Error> {
        // Implement upsert logic
        Ok(None)
    }

    async fn delete(&self, params: Self::QueryParams) -> Result<Option<Entity>, Self::Error> {
        let filter = params.to_query();
        let delete_result = self.collection.find_one_and_delete(filter, None).await?;
        Ok(delete_result)
    }

    async fn update(
        &self, item: &UpdateModel, params: Self::QueryParams,
    ) -> Result<Option<Entity>, Self::Error> {
        let filter = params.to_query();
        let update_result = self
            .collection
            .find_one_and_replace(filter, item, None)
            .await?;
        Ok(update_result)
    }

    async fn select<U>(
        &self, query: U, params: Option<Self::QueryParams>,
    ) -> Result<Vec<Entity>, Self::Error>
    where
        U: AsRef<str> + Send + Sync,
    {
        let filter = params.map_or_else(|| doc! {}, |p| p.to_query());
        let cursor = self.collection.find(filter, None).await?;
        let results: Vec<Entity> = cursor.try_collect().await?;
        Ok(results)
    }
}
