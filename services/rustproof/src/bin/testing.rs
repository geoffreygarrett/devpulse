use rustproof::config::ResendConfig;
use rustproof::services::email_service::{EmailService, ResendConfig, SmtpConfig};

// // use base64::engine::general_purpose::URL_SAFE_NO_PAD;
// // use base64::prelude::*;
// // use rand::{distributions::Alphanumeric, Rng};
// // use sha2::{Digest, Sha256};
// //
// //
// // /// Generates a secure, random refresh token.
// // fn generate_refresh_token() -> String {
// //     // Generate a 32-byte (256-bit) random string
// //     let token: String = rand::thread_rng()
// //         .sample_iter(&Alphanumeric)
// //         .take(32)
// //         .map(char::from)
// //         .collect();
// //
// //     token
// // }
// //
// // /// Hashes a given refresh token using SHA-256.
// // fn hash_refresh_token(token: &str) -> String {
// //     let mut hasher = Sha256::new();
// //     hasher.update(token);
// //     let result = hasher.finalize();
// //
// //     // Encode the hash in a URL-safe base64 string
// //     URL_SAFE_NO_PAD.encode(&result)
// // }
// //
// // fn main() {
// //     // Generate a new refresh token
// //     let refresh_token = generate_refresh_token();
// //     println!("Generated Refresh Token: {}", refresh_token);
// //
// //     // Hash the refresh token for secure storage
// //     let hashed_token = hash_refresh_token(&refresh_token);
// //     println!("Hashed Refresh Token: {}", hashed_token);
// // }
// //
// // // use base64::engine::general_purpose::URL_SAFE_NO_PAD;
// // // use base64::Engine;
// // // use sha2::{Digest, Sha256};
// // // use uuid::Uuid;
// // //
// // // /// Generates a UUID as the refresh token.
// // // fn generate_refresh_token() -> String {
// // //     Uuid::new_v4().to_string()
// // // }
// // //
// // // /// Hashes the refresh token using SHA-256.
// // // fn hash_refresh_token(token: &str) -> String {
// // //     let mut hasher = Sha256::new();
// // //     hasher.update(token);
// // //     let result = hasher.finalize();
// // //     URL_SAFE_NO_PAD.encode(&result)
// // // }
// // //
// // // fn main() {
// // //     // Generate a new refresh token (UUID)
// // //     let refresh_token = generate_refresh_token();
// // //     println!("Generated Refresh Token: {}", refresh_token);
// // //
// // //     // Hash the refresh token for secure storage
// // //     let hashed_token = hash_refresh_token(&refresh_token);
// // //     println!("Hashed Refresh Token: {}", hashed_token);
// // // }
//
// use jsonwebtoken::Validation;
// use std::sync::Arc;
// use rustproof::adapter::Database;
// // #[nject::injectable]
// // struct AuthServiceImpl;
//
// // #[nject::provider]
// // #[provide(Box<dyn AuthService>, |auth_service: AuthServiceImpl| Box::new(auth_service))]
// // struct ServiceProvider {
// //     #[provide(Box<dyn AuthService>)]
// //     auth_service: Box<dyn AuthService>,
// // }
// // pub struct AuthServiceImpl<U, RT, S, SR>
// // where
// //     U: UserRepository + Send + Sync,
// //     RT: RefreshTokenRepository + Send + Sync,
// //     S: TokenService + Send + Sync,
// //     SR: SessionRepository + Send + Sync,
// // {
//
//
// #[nject::provider]
// struct ServiceProvider {
//     #[provide(Arc<dyn TokenService>, |x| x.clone())]
//     auth_token_service: Arc<dyn TokenService>,
//     // #[provide(Arc<dyn AuthService>, |x| x.clone())]
//     // auth_service: Arc<dyn AuthService>,
//
//     #[provide]
//     db: Arc<dyn Database>
// }
//
// use rustproof::adapter::postgres::PostgresDatabase;
// use rustproof::services::auth_service::AuthService;
// use rustproof::services::token_service::auth_token_service::AuthTokenService;
// use rustproof::services::token_service::token_generator::jwt::JwtTokenGenerator;
// use rustproof::services::token_service::TokenService;
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let pool = sqlx::PgPool::connect("postgres://postgres.xijufqypeyrpkypqdzbi:mYxunFHqdzIdmV00@aws-0-eu-west-2.pooler.supabase.com:5432/postgres?options=-c%20search_path=_auth")
//         .await
//         .unwrap();
//
//     let options = sqlx::postgres::PgPoolOptions::new()
//         .max_connections(5);
//
//
//     // Defaults
//     //          // User-specifiable routines
//     //             after_connect: None,
//     //             before_acquire: None,
//     //             after_release: None,
//     //             test_before_acquire: true,
//     //             // A production application will want to set a higher limit than this.
//     //             max_connections: 10,
//     //             min_connections: 0,
//     //             // Logging all acquires is opt-in
//     //             acquire_time_level: LevelFilter::Off,
//     //             // Default to warning, because an acquire timeout will be an error
//     //             acquire_slow_level: LevelFilter::Warn,
//     //             // Fast enough to catch problems (e.g. a full pool); slow enough
//     //             // to not flag typical time to add a new connection to a pool.
//     //             acquire_slow_threshold: Duration::from_secs(2),
//     //             acquire_timeout: Duration::from_secs(30),
//     //             idle_timeout: Some(Duration::from_secs(10 * 60)),
//     //             max_lifetime: Some(Duration::from_secs(30 * 60)),
//     //             fair: true,
//     //             parent_pool: None,
//     let db = Arc::new(PostgresDatabase::new(&pool));
//
//
//     let provider = Provider {
//         auth_token_service: Arc::new(AuthTokenService::new(
//             db.clone(),
//             Arc::new(JwtTokenGenerator::new(
//                 jsonwebtoken::EncodingKey::from_secret("supersecret".as_ref()),
//                 jsonwebtoken::DecodingKey::from_secret("supersecret".as_ref()),
//                 Validation::default(),
//                 3600,
//             )),
//             db.clone(),
//         )),
//     };
//
//
//     //
//     // let ts = AuthTokenService::new(
//     //     db.clone(),
//     //     Arc::new(JwtTokenGenerator::new(
//     //         jsonwebtoken::EncodingKey::from_secret("supersecret".as_ref()),
//     //         jsonwebtoken::DecodingKey::from_secret("supersecret".as_ref()),
//     //         Validation::default(),
//     //         3600,
//     //     )),
//     //     db.clone(),
//     // );
//     // let _facade: AuthServiceImpl<
//     //     PostgresDatabase,
//     //     PostgresDatabase,
//     //     AuthTokenService,
//     //     PostgresDatabase
//     // > = Provider.provide().await?;
//     // AuthTokenService
//     // let service_provider = ServiceProvider {
//     //     auth_service: AuthServiceImpl<>
//     // };
//     // let config = Config {
//     //     jwt_secret: "supersecret".to_string(),
//     //     jwt_exp: 3600,
//     // };
//     //
//     // let user_service = Arc::new(UserService::new(config));
//     //
//     // let http_router = create_http_router(user_service.clone());
//     // let grpc_service = run_grpc_server(user_service.clone());
//     //
//     // // Run both servers concurrently
//     // let http_server = axum::Server::bind(&"127.0.0.1:3000".parse()?)
//     //     .serve(http_router.into_make_service());
//     //
//     // join!(http_server, grpc_service);
//
//     Ok(())
// }
#[tokio::main]
async fn main() {
    let resend_config = ResendConfig {
        api_key: "re_K3MhWRfR_686rXChwJCGhwpYEitnk1teY".to_string().parse().unwrap(),
        from_name: "Geoffrey Garrett".to_string(),
        from_email: "verify@geoffreygarrett.com".to_string(),
        sender_name: Some("Geoffrey Garrett".to_string()),
    };

    let email_config: SmtpConfig = resend_config.into();
    let email_service = EmailService::new(email_config.into());

    // match email_service.send_email(
    //     "Geoffrey Garrett",
    //     "g.h.garrett13@gmail.com",
    //     "Test Subject",
    //     "This is a test email body.",
    // ) {
    //     Ok(_) => println!("Email sent successfully!"),
    //     Err(e) => eprintln!("Failed to send email: {:?}", e),
    // }

    match email_service.send_verification_email(
        "Geoffrey Garrett",
        "g.h.garrett13@gmail.com",
        "https://example.com/verify",
    ) {
        Ok(_) => println!("Verification email sent successfully!"),
        Err(e) => eprintln!("Failed to send verification email: {:?}", e),
    }
}

// use rustproof::services::hibp_service::{HIBPError, HIBPService};
//
// #[tokio::main]
// async fn main() {
//     let hibp_service = HIBPService::new(Some("your-api-key".to_string()), "your-app-name".to_string());
//
//     // Example of checking if a password has been breached
//     match hibp_service.check_password("your-password").await {
//         Ok(_) => println!("Password is safe"),
//         Err(HIBPError::PasswordBreached { count }) => println!("Password has been found in breaches {} times", count),
//         Err(err) => println!("An error occurred: {}", err),
//     }
//
//     // Example of getting breaches for an account
//     match hibp_service.get_breaches_for_account("example@example.com").await {
//         Ok(breaches) => {
//             for breach in breaches {
//                 println!("{:?}", breach);
//             }
//         }
//         Err(err) => println!("An error occurred: {}", err),
//     }
// }
