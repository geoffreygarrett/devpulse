use async_trait::async_trait;

use crate::adapters::postgres::PgPool;
// use crate::adapters::postgres::parse_timestamp;
use crate::adapters::traits::convertible::Convertible;
use crate::adapters::traits::dao::Dao;
use crate::proto::db_auth_v1::AccountInsert;
use crate::proto::db_auth_v1::AccountModel;
use crate::proto::db_auth_v1::AccountUpdate;

use super::filters::{QueryParams, ToSqlString};

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Internal server error")]
    InternalError(String),
}

impl From<ServiceError> for tonic::Status {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::DatabaseError(db_err) => match db_err {
                sqlx::Error::RowNotFound => tonic::Status::not_found("Row not found"),
                sqlx::Error::PoolTimedOut => {
                    tonic::Status::unavailable("Database connection pool timed out")
                }
                _ => tonic::Status::internal(format!("Database error: {:?}", db_err)),
            },
            ServiceError::InternalError(msg) => tonic::Status::internal(msg),
        }
    }
}

#[async_trait]
impl Dao<AccountModel, AccountInsert, AccountUpdate> for PgPool {
    type Error = sqlx::Error;
    type QueryParams = QueryParams;

    async fn insert(
        &self, item: &AccountInsert, params: Option<Self::QueryParams>,
    ) -> Result<Option<AccountModel>, Self::Error> {
        let params = params.unwrap_or_default();
        let returning_clause = params.build_returning_clause();

        let query = format!(
            "INSERT INTO accounts (uuid, given_name, email, hash, avatar_url) VALUES ($1, $2, $3, $4, $5){}",
            returning_clause
        );

        sqlx::query_as::<_, AccountModel>(&query)
            .bind(&item.uuid)
            .bind(&item.given_name)
            .bind(&item.email)
            .bind(&item.hash)
            .bind(&item.avatar_url)
            .fetch_optional(self)
            .await
            .map_err(Into::into)
    }

    async fn update(
        &self, item: &AccountUpdate, params: Self::QueryParams,
    ) -> Result<Option<AccountModel>, Self::Error> {
        let where_clause = params.build_where_clause();
        let returning_clause = params.build_returning_clause();

        let mut set_clauses = Vec::new();
        if let Some(given_name) = &item.given_name {
            set_clauses.push(format!("given_name = '{}'", given_name));
        }
        if let Some(email) = &item.email {
            set_clauses.push(format!("email = '{}'", email));
        }
        if let Some(hash) = &item.hash {
            set_clauses.push(format!("hash = '{}'", hash));
        }
        if let Some(avatar_url) = &item.avatar_url {
            set_clauses.push(format!("avatar_url = '{}'", avatar_url));
        }

        if set_clauses.is_empty() {
            return Err(sqlx::Error::ColumnNotFound("No fields provided for update".into()));
        }

        let query = format!(
            "UPDATE accounts SET {}{}{}",
            set_clauses.join(", "),
            where_clause,
            returning_clause
        );

        sqlx::query_as::<_, AccountModel>(&query)
            .fetch_optional(self)
            .await
            .map_err(Into::into)
    }

    async fn upsert(
        &self, item: &AccountInsert, conflict_resolution: &str, params: Option<Self::QueryParams>,
    ) -> Result<Option<AccountModel>, Self::Error> {
        let params = params.unwrap_or(QueryParams::new(Vec::new(), None));
        let where_clause = params.build_where_clause();
        let returning_clause = params.build_returning_clause();

        let query = format!(
            "INSERT INTO accounts (uuid, given_name, email, hash, avatar_url, created_at)
            VALUES ($1::uuid, $2, $3, $4, $5, NOW())
            ON CONFLICT ({}) DO UPDATE
            SET uuid = EXCLUDED.uuid, given_name = EXCLUDED.given_name, email = EXCLUDED.email, hash = EXCLUDED.hash, avatar_url = EXCLUDED.avatar_url
            {} {}",
            conflict_resolution, where_clause, returning_clause
        );

        sqlx::query_as::<_, AccountModel>(&query)
            .bind(&item.uuid)
            .bind(&item.given_name)
            .bind(&item.email)
            .bind(&item.hash)
            .bind(&item.avatar_url)
            .fetch_optional(&*self)
            .await
            .map_err(Into::into)
    }

    async fn delete(&self, params: Self::QueryParams) -> Result<Option<AccountModel>, Self::Error> {
        let where_clause = params.build_where_clause();
        let returning_clause = build_returning_clause();

        let query = format!("DELETE FROM accounts{}{}", where_clause, returning_clause);

        sqlx::query_as::<_, AccountModel>(&query)
            .fetch_optional(self)
            .await
            .map_err(Into::into)
    }

    async fn select<U>(
        &self, query: U, params: Option<Self::QueryParams>,
    ) -> Result<Vec<AccountModel>, Self::Error>
    where
        U: AsRef<str> + Send + Sync,
    {
        let query_str = query.as_ref();
        let returns = params
            .and_then(|p| p.returns())
            .unwrap_or_else(|| "*".to_string());

        let query = format!("SELECT {} FROM accounts {}", returns, query_str);

        let result = sqlx::query_as::<_, AccountModel>(&query)
            .fetch_all(self)
            .await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::adapters::postgres::filters::FilterBuilder;
    use crate::adapters::traits::DbPool;

    use super::*;

    async fn setup_test_db() -> anyhow::Result<PgPool> {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgPool::new_pool(&database_url).await
    }

    #[tokio::test]
    async fn test_postgres() {
        let pool = setup_test_db().await.unwrap();
        let dao = &pool;
        let uuid = "00000000-0000-0000-0000-000000000001";
        //
        // let account = AccountInsert {
        //     uuid: Some(uuid.to_string()),
        //     given_name: Option::from("Test".to_string()),
        //     email: Option::from("b@gmail.com".to_string()),
        //     hash: Option::from("hash".to_string()),
        //     avatar_url: Option::from("https://example.com/avatar".to_string()),
        // };
        //
        // let result = dao.insert(&account, None).await.unwrap();
        //
        // assert_eq!(result.is_some(), true);
        // let account = result.unwrap();
        // assert_eq!(account.uuid, uuid);
        // assert_eq!(account.given_name, "Test");

        // delete
        let uuid_filter = format!("'{}'::uuid", uuid);
        let params =
            QueryParams::new(FilterBuilder::new().equal("uuid", &uuid_filter).build(), None);
        let result = dao.delete(params).await.unwrap();
        result.unwrap();
    }
}
