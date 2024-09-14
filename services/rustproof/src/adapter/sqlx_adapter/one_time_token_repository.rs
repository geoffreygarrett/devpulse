// use crate::adapter::{DbPool, GenericRepository, OneTimeToken, OneTimeTokenRepository};
// use crate::errors::{DatabaseSnafu, Result};
// use async_trait::async_trait;
// use snafu::ResultExt;
// use sqlx::{Database, Executor, Pool};
// use uuid::Uuid;
//
// /// A generic implementation of the `OneTimeTokenRepository` trait for any SQL database
// /// supported by `sqlx`. This allows the repository to be used with PostgreSQL, MySQL,
// /// SQLite, and others.
// ///
// /// The `GenericRepository` is initialized with a connection pool, which is then
// /// used to execute database queries.
// ///
// /// # Type Parameters
// /// - `DB`: The specific database type, such as `Postgres`, `MySql`, or `Sqlite`.
// #[async_trait]
// impl<DB> OneTimeTokenRepository for GenericRepository<Pool<DB>>
// where
//     DB: Database,
//     for<'c> &'c Pool<DB>: Executor<'c, Database = DB>, for<'c> Uuid: sqlx::Encode<'c, DB> // Ensures compatibility with the executor
// {
//     /// Stores a one-time token in the database.
//     ///
//     /// # Parameters
//     /// - `token`: The `OneTimeToken` to be stored.
//     ///
//     /// # Errors
//     /// Returns a `DatabaseSnafu` error if the SQL execution fails.
//     async fn store_one_time_token(&self, token: OneTimeToken) -> Result<()> {
//         // language=SQL
//         sqlx::query(
//             r#"
//                 INSERT INTO one_time_tokens (id, user_id, token_type, token, metadata, created_at, expires_at, used, revoked)
//                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
//             "#
//         )
//             .bind(token.id)
//             .bind(token.user_id)
//             .bind(token.token_type)
//             .bind(token.token)
//             .bind(token.metadata)
//             .bind(token.created_at)
//             .bind(token.expires_at)
//             .bind(token.used)
//             .bind(token.revoked)
//             .execute(&self.pool)
//             .await
//             .context(DatabaseSnafu)?;
//
//         Ok(())
//     }
//
//     /// Validates a one-time token and returns the associated `OneTimeToken` record if valid.
//     ///
//     /// # Parameters
//     /// - `token`: The token string to validate.
//     /// - `token_type`: The type of the token.
//     ///
//     /// # Errors
//     /// Returns a `DatabaseSnafu` error if the SQL execution fails.
//     async fn validate_one_time_token(&self, token: &str, token_type: &str) -> Result<OneTimeToken> {
//         // language=SQL
//         let token = sqlx::query_as::<_, OneTimeToken>(
//             r#"
//                 SELECT id, user_id, token_type, token, metadata, created_at, expires_at, used, revoked
//                 FROM one_time_tokens
//                 WHERE token = ? AND token_type = ? AND revoked = FALSE AND used = FALSE AND expires_at > NOW()
//             "#
//         )
//             .bind(token.to_string())
//             .bind(token_type.to_string())
//             .fetch_one(&self.pool)
//             .await
//             .context(DatabaseSnafu)?;
//
//         Ok(token)
//     }
//
//     /// Revokes a one-time token by marking it as revoked in the database.
//     ///
//     /// # Parameters
//     /// - `token`: The token string to revoke.
//     ///
//     /// # Errors
//     /// Returns a `DatabaseSnafu` error if the SQL execution fails.
//     async fn revoke_one_time_token(&self, token: &str) -> Result<()> {
//         // language=SQL
//         sqlx::query(
//             r#"
//                 UPDATE one_time_tokens
//                 SET revoked = TRUE
//                 WHERE token = ?
//             "#
//         )
//             .bind(token.to_string())
//             .execute(&self.pool)
//             .await
//             .context(DatabaseSnafu)?;
//
//         Ok(())
//     }
// }
