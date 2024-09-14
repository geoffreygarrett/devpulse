use tracing::{info, error};
use tracing_subscriber;
use crate::migration::run_migrations;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Initialize tracing subscriber with environment variable filter
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    info!("Starting migrations");

    if let Err(e) = run_migrations().await {
        error!("Migration failed: {:?}", e);
        return Err(e);
    }

    Ok(())
}
