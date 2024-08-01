
use anyhow::{Context, Result};
use std::net::SocketAddr;
use std::time::Duration;
use tonic::transport::Server;

mod v1 {
    use anyhow::{Context, Result};
    use devpulse_db::adapters::postgres::PgPool;
    pub use devpulse_db::services::v1::account::AccountServiceImpl;
    // pub use crate::services::v1::identity_service::IdentityServiceImpl;
    // pub use crate::services::v1::refresh_token_service::RefreshTokenServiceImpl;
    pub use devpulse_db::services::v1::account::AccountServiceServer;
    use sqlx::postgres::PgPoolOptions;

    pub struct Services {
        pub account_service: AccountServiceImpl<PgPool>,
        // pub identity_service: IdentityServiceImpl,
        // pub refresh_token_service: RefreshTokenServiceImpl,
    }

    impl Services {
        pub async fn new() -> Result<Self> {
            let database_url = std::env::var("DATABASE_URL")
                .context("DATABASE_URL environment variable not set")?;

            let pool = PgPoolOptions::new()
                .max_connections(20) // Increase the number of maximum connections
                .acquire_timeout(std::time::Duration::from_secs(30)) // Increase the timeout duration
                .connect(&database_url)
                .await
                .context("Failed to create PgPool")?;

            Ok(Services {
                account_service: AccountServiceImpl {
                    data_accessor: pool,
                },
            })
        }
    }
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // check if the DATABASE_URL environment variable is set
//
//     // let addr = "[::1]:50051".parse()?;
//     let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 50052));
//     let services = v1::Services::new().await?;
//
//     //
//     // println!("gRPC server listening on {}", addr);
//     //
//     // Server::builder()
//     //     .add_service(v1::AccountServiceServer::new(services.account_service))
//     //     // .add_service(v1::IdentityServiceServer::new(services.identity_service))
//     //     // .add_service(v1::RefreshTokenServiceServer::new(services.refresh_token_service))
//     //     .serve(addr)
//     //     .await?;
//
//     // let new_account = AccountCreate::default();
//     // println!("new account: {:?}", new_account.sql_insert());
//
//     Ok(())
// }


pub mod proto {
    pub(crate) const FILE_DESCRIPTOR_SET_V1: &[u8] = tonic::include_file_descriptor_set!("db.v1");
}

// static MIGRATOR: sqlx::Migrator = sqlx::migrate!("./migrations");


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let mut services = v1::Services::new().await?;
    sqlx::migrate!("./migrations")
        .run(&services.account_service.data_accessor)
        .await?;

    let addr = "[::1]:50051".parse()?;
    // Example: Running a dummy server (replace with your actual server code)
    tokio::spawn(async move {
        loop {
            println!("Server is running...");
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });
    Server::builder()
        .add_service(
            tonic_reflection::server::Builder::configure()
                .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET_V1)
                .build()
                .unwrap(),
        )
        .add_service(v1::AccountServiceServer::new(services.account_service))
            // .add_service(v1::IdentityServiceServer::new(services.identity_service))
            // .add_service(v1::RefreshTokenServiceServer::new(services.refresh_token_service))
            .serve(addr)
            .await?;

    //
    // // Spawn a task to check the database connection every 10 seconds
    // tokio::spawn(async move {
    //     let mut interval = interval(Duration::from_secs(10));
    //     loop {
    //         interval.tick().await;
    //         match services.account_service.data_accessor.().await {
    //             Ok(_) => println!("Database connection is healthy"),
    //             Err(e) => eprintln!("Database connection failed: {}", e),
    //         }
    //     }
    // });

    // Your other application logic here
    // let addr = "[::1]:50051".parse()?;
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 50052));

    // Keep the main function alive
    tokio::signal::ctrl_c().await?;
    println!("Shutting down");

    Ok(())
}
