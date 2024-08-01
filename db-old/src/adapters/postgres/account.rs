// use async_trait::async_trait;
// use sqlx::Error as SqlxError;
//
// use crate::adapters::postgres::{parse_uuid, PgPool};
// // use crate::adapters::postgres::parse_timestamp;
// use crate::adapters::traits::convertible::Convertible;
// use crate::adapters::traits::dao::Dao;
// use crate::proto::db_auth_v1::Account;
//
// use super::filters::{QueryParams, ToSqlString};
//
//
// // impl<'r> FromRow<'r, PgRow> for Account {
// //     fn from_row(row: &'r PgRow) -> Result<Self, Error> {
// //         let uuid_str: String = row.try_get("uuid")?;
// //         let uuid = InternalUuid::try_from(uuid_str).map_err(|_| Error::ColumnDecode {
// //             index: "uuid".to_string(),
// //             source: Box::new(ConversionError::InvalidUuidString),
// //         })?;
// //
// //         let created_at_opt: Option<InternalTimestamp> = row.try_get("created_at")?;
// //         let created_at: Option<InternalTimestamp> = created_at_opt
// //             .map(|dt| prost_types::Timestamp {
// //                 seconds: dt.0.timestamp(),
// //                 nanos: dt.0.timestamp_subsec_nanos() as i32,
// //             })
// //             .map(|ts| InternalTimestamp::try_from(ts))
// //             .transpose()
// //             .map_err(|_| Error::ColumnDecode {
// //                 index: "created_at".to_string(),
// //                 source: Box::new(ConversionError::InvalidTimestamp),
// //             })?;
// //
// //         Ok(Self {
// //             id: row.try_get("id")?,
// //             uuid: String::from(uuid.0),
// //             given_name: row.try_get("given_name")?,
// //             email: row.try_get("email")?,
// //             hash: row.try_get("hash")?,
// //             avatar_url: row.try_get("avatar_url")?,
// //             created_at:
// //         })
// //     }
// // }
//
// #[async_trait]
// impl Dao<Account> for PgPool {
//     type Error = SqlxError;
//     type QueryParams = QueryParams;
//
//     async fn insert(
//         &self, account: &Account, params: Option<Self::QueryParams>,
//     ) -> Result<Option<Account>, Self::Error> {
//         let returns = params
//             .and_then(|p| p.returns())
//             .unwrap_or_else(|| "*".to_string());
//
//         let query = format!(
//             "INSERT INTO accounts (uuid, given_name, email, hash, avatar_url, created_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING {}",
//             returns
//         );
//
//         let result = sqlx::query_as::<_, Account>(&query)
//             .bind(parse_uuid(&account.uuid))
//             .bind(&account.given_name)
//             .bind(&account.email)
//             .bind(&account.hash)
//             .bind(&account.avatar_url)
//             .bind(&account.created_at)
//             .fetch_optional(self)
//             .await?;
//
//         Ok(result)
//     }
//
//     async fn upsert(
//         &self, account: &Account, conflict_resolution: &str, params: Option<Self::QueryParams>,
//     ) -> Result<Option<Account>, Self::Error> {
//         let params = params.unwrap_or(QueryParams::new(None, Some("*".to_string())));
//         let filters_str = params
//             .filters()
//             .unwrap_or_else(Vec::new)
//             .iter()
//             .map(|f| f.to_sql_string())
//             .collect::<Vec<_>>()
//             .join(" AND ");
//         let returns = params.returns().unwrap_or_else(|| "*".to_string());
//
//         let query = format!(
//             "INSERT INTO accounts (uuid, given_name, email, hash, avatar_url, created_at) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT ({}) DO UPDATE SET uuid = EXCLUDED.uuid, given_name = EXCLUDED.given_name, email = EXCLUDED.email, hash = EXCLUDED.hash, avatar_url = EXCLUDED.avatar_url, created_at = EXCLUDED.created_at WHERE {} RETURNING {}",
//             conflict_resolution, filters_str, returns
//         );
//
//         let result = sqlx::query_as::<_, Account>(&query)
//             .bind(&account.uuid)
//             .bind(&account.given_name)
//             .bind(&account.email)
//             .bind(&account.hash)
//             .bind(&account.avatar_url)
//             .bind(&account.created_at)
//             .fetch_optional(self)
//             .await?;
//
//         Ok(result)
//     }
//
//     async fn delete(&self, params: Self::QueryParams) -> Result<Option<Account>, Self::Error> {
//         let filters_str = params
//             .filters()
//             .unwrap_or_else(Vec::new)
//             .iter()
//             .map(|f| f.to_sql_string())
//             .collect::<Vec<_>>()
//             .join(" AND ");
//         let returns = params.returns().unwrap_or_else(|| "*".to_string());
//
//         let query = format!("DELETE FROM accounts WHERE {} RETURNING {}", filters_str, returns);
//
//         let result = sqlx::query_as::<_, Account>(&query)
//             .fetch_optional(self)
//             .await?;
//
//         Ok(result)
//     }
//
//     async fn update(
//         &self, account: &Account, params: Self::QueryParams,
//     ) -> Result<Option<Account>, Self::Error> {
//         let filters_str = params
//             .filters()
//             .unwrap_or_else(Vec::new)
//             .iter()
//             .map(|f| f.to_sql_string())
//             .collect::<Vec<_>>()
//             .join(" AND ");
//         let returns = params.returns().unwrap_or_else(|| "*".to_string());
//
//         let query = format!(
//             "UPDATE accounts SET uuid = $1, given_name = $2, email = $3, hash = $4, avatar_url = $5, created_at = $6 WHERE {} RETURNING {}",
//             filters_str, returns
//         );
//
//         let result = sqlx::query_as::<_, Account>(&query)
//             .bind(&account.uuid)
//             .bind(&account.given_name)
//             .bind(&account.email)
//             .bind(&account.hash)
//             .bind(&account.avatar_url)
//             .bind(&account.created_at)
//             .fetch_optional(self)
//             .await?;
//
//         Ok(result)
//     }
//
//     async fn select<U>(
//         &self, query: U, params: Option<Self::QueryParams>,
//     ) -> Result<Vec<Account>, Self::Error>
//     where
//         U: AsRef<str> + Send + Sync,
//     {
//         let query_str = query.as_ref();
//         let returns = params
//             .and_then(|p| p.returns())
//             .unwrap_or_else(|| "*".to_string());
//
//         let query = format!("SELECT {} FROM accounts {}", returns, query_str);
//
//         let result = sqlx::query_as::<_, Account>(&query).fetch_all(self).await?;
//
//         Ok(result)
//     }
// }
