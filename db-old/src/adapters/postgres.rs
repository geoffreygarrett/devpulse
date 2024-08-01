use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use ::uuid::Uuid;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{Database, Postgres};
pub use sqlx::{FromRow, PgPool};
use sqlx::error::BoxDynError;
use tonic::Status;

use crate::adapters::traits::data_access::{DataAccess, DbPool};
use crate::proto::db_auth_v1;
use crate::proto::db_auth_v1::{AccountCreate, AccountUpdate};
use crate::proto::db_auth_v1::Account;

mod account;
mod filters;
mod types;
mod uuid;
// impl From<db_auth_v1::Uuid> for sqlx::types::Uuid {
//     fn from(uuid: db_auth_v1::Uuid) -> Self {
//         uuid.value
//     }
// }
//
// impl From<sqlx::types::Uuid> for db_auth_v1::Uuid {
//     fn from(uuid: sqlx::types::Uuid) -> Self {
//         db_auth_v1::Uuid {
//             value: uuid,
//         }
//     }
// }

impl From<DataAccessError> for Status {
    fn from(err: DataAccessError) -> Self {
        match err {
            DataAccessError::SqlError(_) => Status::internal("Database error"),
            DataAccessError::NotFound => Status::not_found("Resource not found"),
            DataAccessError::MissingUuid => Status::invalid_argument("Missing UUID"),
            DataAccessError::InvalidUuidFormat => Status::invalid_argument("Invalid UUID format"),
            DataAccessError::MissingField(field) => {
                Status::invalid_argument(format!("Missing field: {}", field))
            }
        }
    }
}

#[async_trait]
impl DbPool for PgPool {
    type Pool = PgPool;

    async fn new_pool(database_url: &str) -> Result<Self::Pool> {
        let pool = PgPool::connect(database_url).await?;
        Ok(pool)
    }
}
#[derive(Debug)]
pub enum DataAccessError {
    SqlError(sqlx::Error),
    NotFound,
    MissingUuid,
    InvalidUuidFormat,
    MissingField(&'static str),
}

impl Display for DataAccessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DataAccessError::SqlError(err) => write!(f, "Database error: {}", err),
            DataAccessError::NotFound => write!(f, "Resource not found"),
            DataAccessError::MissingUuid => write!(f, "Missing UUID"),
            DataAccessError::InvalidUuidFormat => write!(f, "Invalid UUID format"),
            DataAccessError::MissingField(field) => write!(f, "Missing field: {}", field),
        }
    }
}

impl From<sqlx::Error> for DataAccessError {
    fn from(err: sqlx::Error) -> Self {
        DataAccessError::SqlError(err)
    }
}

impl Error for DataAccessError {}

fn parse_optional_uuid(uuid_str: Option<String>) -> Option<Uuid> {
    match uuid_str {
        Some(uuid) => Uuid::from_str(&uuid).ok(),
        None => None,
    }
}

fn parse_uuid(uuid_str: &str) -> Uuid {
    Uuid::from_str(uuid_str).unwrap_or_else(|_| Uuid::new_v4())
}

// fn parse_timestamp(timestamp: Timestamp) -> Result<sqlx::types::chrono::NaiveDateTime, SqlxError> {
//     let seconds = timestamp.seconds;
//     let nanos = timestamp.nanos;
//     let duration = std::time::Duration::new(seconds as u64, nanos as u32);
//     Ok(sqlx::types::chrono::NaiveDateTime::from_timestamp(
//         duration.as_secs() as i64,
//         duration.subsec_nanos(),
//     ))
// }

// impl<'r> sqlx::Decode<'r, Postgres> for db_auth_v1::Uuid {
//     fn decode(value: <Postgres as sqlx::Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
//         Ok(db_auth_v1::Uuid {
//             value: sqlx::types::Uuid::decode(value)?.into(),
//         })
//     }
// }
//
impl<'r> sqlx::Decode<'r, Postgres> for db_auth_v1::Timestamp {
    fn decode(value: <Postgres as sqlx::Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        let timestamp = sqlx::types::chrono::NaiveDateTime::decode(value)?;
        Ok(db_auth_v1::Timestamp {
            value: Some(prost_types::Timestamp {
                seconds: timestamp.timestamp(),
                nanos: timestamp.timestamp_subsec_nanos() as i32,
            }),
        })
    }
}

impl sqlx::Type<Postgres> for db_auth_v1::Timestamp {
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::chrono::NaiveDateTime as sqlx::Type<Postgres>>::type_info()
    }

    fn compatible(ty: &<Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::chrono::NaiveDateTime as sqlx::Type<Postgres>>::compatible(ty)
    }
}

// impl sqlx::Type<Postgres> for db_auth_v1::Uuid {
//     fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
//         <sqlx::types::Uuid as sqlx::Type<Postgres>>::type_info()
//     }
//
//     fn compatible(ty: &<Postgres as sqlx::Database>::TypeInfo) -> bool {
//         <sqlx::types::Uuid as sqlx::Type<Postgres>>::compatible(ty)
//     }
// }

// impl<'q> sqlx::Encode<'q, Postgres> for db_auth_v1::Uuid {
//     fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
//         sqlx::types::Uuid::from_str(&self.value)?.encode(buf)
//     }
//
//     fn encode_by_ref(
//         &self, buf: &mut <Postgres as Database>::ArgumentBuffer<'q>,
//     ) -> std::result::Result<IsNull, BoxDynError> {
//         sqlx::types::Uuid::from_str(&self.value)?.encode(buf)
//     }
//
//     fn size_hint(&self) -> usize {
//         sqlx::types::Uuid::from_str(&self.value)
//             .unwrap()
//             .size_hint()
//     }
// }

macro_rules! bind_uuid {
    ($uuid:expr) => {
        Some(Uuid::from_str(&$uuid).map_err(|_| DataAccessError::InvalidUuidFormat)?)
    };
}

// "deleted_at!: Option<DateTime<Local>>"

#[async_trait]
impl DataAccess<Account, AccountCreate, AccountUpdate> for PgPool {
    type Error = DataAccessError;
    async fn create(&self, item: &AccountCreate) -> Result<Account, Self::Error> {
        sqlx::query_as::<_, Account>(
            r#"
            INSERT INTO accounts (uuid, given_name, email, hash, avatar_url)
            VALUES ($1::uuid, $2, $3, $4, $5)
            RETURNING id, uuid, given_name, email, hash, avatar_url, created_at as "created_at!: alloc::string::String"
            "#,
        )
        .bind(&item.uuid)
        // .bind(bind_uuid!(item.uuid))
        .bind(&item.given_name)
        .bind(&item.email)
        .bind(&item.hash)
        .bind(&item.avatar_url)
        .fetch_one(self)
        .await
        .map_err(DataAccessError::from)
    }

    async fn read(&self, id: u64) -> Result<Option<Account>, Self::Error> {
        let result = sqlx::query_as::<_, Account>(
            r#"SELECT id, uuid, given_name, email, hash, avatar_url, created_at
            FROM accounts
            WHERE id = $1"#,
        )
        .bind(id as i32)
        .fetch_optional(self)
        .await
        .map_err(DataAccessError::from)?;
        Ok(result)
    }

    async fn update(&self, item: &AccountUpdate) -> Result<Account, Self::Error> {
        sqlx::query_as::<_, Account>(
            r#"UPDATE accounts SET uuid = $1, given_name = $2, email = $3, hash = $4, avatar_url = $5
            WHERE id = $6"#)
            .bind(&item.uuid)
            .bind(&item.given_name)
            .bind(&item.email)
            .bind(&item.hash)
            .bind(&item.avatar_url)
            .bind(item.id)
            .fetch_one(self)
            .await
            .map_err(DataAccessError::from)
    }

    async fn delete(&self, id: u64) -> Result<(), Self::Error> {
        let result = sqlx::query("DELETE FROM accounts WHERE id = $1")
            .bind(id as i32)
            .execute(self)
            .await
            .map_err(DataAccessError::from)?;

        if result.rows_affected() == 0 {
            Err(DataAccessError::NotFound)
        } else {
            Ok(())
        }
    }

    async fn list(&self) -> Result<Vec<Account>, Self::Error> {
        sqlx::query_as::<_, Account>(
            "SELECT id, uuid, given_name, email, hash, avatar_url, created_at FROM accounts",
        )
        .fetch_all(self)
        .await
        .map_err(DataAccessError::from)
    }
}
