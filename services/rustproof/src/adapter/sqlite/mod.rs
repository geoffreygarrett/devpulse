// pub mod user_repository;
// pub mod refresh_token_repository;
// pub mod session_repository;
pub mod password_history_repository;
pub mod one_time_token_repository;

pub use crate::adapter::postgres::{
    one_time_token_repository::*,
    password_history_repository::*,
    refresh_token_repository::*,
    session_repository::*,
    user_repository::*,
};

use crate::adapter::prelude::*;

use sqlx::SqlitePool;

#[async_trait]
impl DbPool for SqlitePool {}
