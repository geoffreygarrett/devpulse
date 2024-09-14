use crate::{DatabaseConnectionSnafu, ServiceError};
use rustproof::config::DatabaseConfig;
use secrecy::ExposeSecret;
use snafu::ResultExt;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Executor, PgConnection, PgPool};
use std::sync::Arc;

#[tracing::instrument(skip(config))]
pub(crate) async fn init_db_pool(config: DatabaseConfig) -> Result<PgPool, ServiceError> {
    let config_for_pool = Arc::new(config);
    let config_for_connect = Arc::clone(&config_for_pool); // Clone the Arc before moving it into the closure

    PgPoolOptions::new()
        .max_connections(config_for_pool.max_connections)
        .after_connect(move |conn: &mut PgConnection, _meta| {
            let local_config = Arc::clone(&config_for_pool); // Correctly clone the Arc
            Box::pin(async move {
                if let Some(namespace) = local_config.namespace.as_deref() {
                    if !namespace.is_empty() {
                        conn.execute(&*format!("SET search_path TO {}", namespace))
                            .await
                            .map_err(|e| {
                                tracing::error!("Failed to set search path: {}", e);
                                e
                            })?;
                    }
                }
                Ok(())
            })
        })
        .connect(config_for_connect.connection_string.expose_secret())
        .await
        .context(DatabaseConnectionSnafu)
}