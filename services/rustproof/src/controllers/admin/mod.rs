use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Json},
    Extension,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::repositories::UserRepository;

#[derive(Deserialize, Default)]
pub struct Pagination {
    #[serde(default = "default_page")]
    pub page: Option<u32>,
    #[serde(default = "default_per_page")]
    pub per_page: Option<u32>,
}

fn default_page() -> Option<u32> {
    Some(1)
}

fn default_per_page() -> Option<u32> {
    Some(50)
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub users: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub per_page: u32,
}

pub async fn list_users(
    Query(pagination): Query<Pagination>,
    Extension(user_repo): Extension<Arc<dyn UserRepository + Send + Sync>>,
) -> impl IntoResponse {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(100);

    match user_repo.list_users(page as i32, per_page as i32).await {
        Ok((users, total)) => {
            let response = PaginatedResponse {
                users,
                total,
                page,
                per_page,
            };
            Json(response).into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_user(
    Path(user_id): Path<Uuid>,
    Extension(user_repo): Extension<Arc<dyn UserRepository + Send + Sync>>,
) -> impl IntoResponse {
    match user_repo.get_user_by_id(&user_id).await {
        Ok(user) => Json(user).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}


pub async fn update_user(
    Path(user_id): Path<Uuid>,
    Extension(user_repo): Extension<Arc<dyn UserRepository + Send + Sync>>,
    Json(payload): Json<UpdateUserPayload>,
) -> impl IntoResponse {
    match user_repo.update_user(&user_id, payload.password_hash.clone()).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_user(
    Path(user_id): Path<Uuid>,
    Extension(user_repo): Extension<Arc<dyn UserRepository + Send + Sync>>,
) -> impl IntoResponse {
    match user_repo.delete_user(&user_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[derive(Deserialize)]
pub struct UpdateUserPayload {
    pub password_hash: String,
}
