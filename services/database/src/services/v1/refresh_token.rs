use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool, Row};
use sqlx::postgres::PgRow;
use tonic::{Request, Response, Status};

use crate::refresh_token::v1::{
    OperationStatus, refresh_token_service_server::RefreshTokenService, RefreshTokenDeleteRequest,
    RefreshTokenFilterCondition, RefreshTokenInsertRequest, RefreshTokenModel,
    RefreshTokenSelectRequest, RefreshTokensResponse, RefreshTokenUpdateRequest,
};

pub struct PostgresRefreshTokenService {
    pub pool: Arc<PgPool>,
}

impl PostgresRefreshTokenService {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            pool: Arc::new(pool.clone()),
        }
    }

    fn build_filter_query(filters: &[RefreshTokenFilterCondition]) -> String {
        filters
            .iter()
            .map(|filter| match &filter.condition {
                Some(crate::refresh_token::v1::refresh_token_filter_condition::Condition::IdFilter(filter)) => {
                    match filter.r#type {
                        Some(crate::refresh_token::v1::id_filter::Type::Equals(value)) => format!("id = {}", value),
                        Some(crate::refresh_token::v1::id_filter::Type::NotEquals(value)) => format!("id != {}", value),
                        Some(crate::refresh_token::v1::id_filter::Type::GreaterThan(value)) => format!("id > {}", value),
                        Some(crate::refresh_token::v1::id_filter::Type::LessThan(value)) => format!("id < {}", value),
                        None => String::new(),
                    }
                }
                Some(crate::refresh_token::v1::refresh_token_filter_condition::Condition::AccountIdFilter(filter)) => {
                    match filter.r#type {
                        Some(crate::refresh_token::v1::account_id_filter::Type::Equals(value)) => format!("account_id = {}", value),
                        Some(crate::refresh_token::v1::account_id_filter::Type::NotEquals(value)) => format!("account_id != {}", value),
                        Some(crate::refresh_token::v1::account_id_filter::Type::GreaterThan(value)) => format!("account_id > {}", value),
                        Some(crate::refresh_token::v1::account_id_filter::Type::LessThan(value)) => format!("account_id < {}", value),
                        None => String::new(),
                    }
                }
                Some(crate::refresh_token::v1::refresh_token_filter_condition::Condition::IssuedAtFilter(filter)) => {
                    match filter.r#type {
                        Some(crate::refresh_token::v1::issued_at_filter::Type::Equals(ref value)) => format!("issued_at = '{}'", value),
                        Some(crate::refresh_token::v1::issued_at_filter::Type::NotEquals(ref value)) => format!("issued_at != '{}'", value),
                        Some(crate::refresh_token::v1::issued_at_filter::Type::Before(ref value)) => format!("issued_at < '{}'", value),
                        Some(crate::refresh_token::v1::issued_at_filter::Type::After(ref value)) => format!("issued_at > '{}'", value),
                        None => String::new(),
                    }
                }
                Some(crate::refresh_token::v1::refresh_token_filter_condition::Condition::ExpiresFilter(filter)) => {
                    match filter.r#type {
                        Some(crate::refresh_token::v1::expires_filter::Type::Equals(ref value)) => format!("expires = '{}'", value),
                        Some(crate::refresh_token::v1::expires_filter::Type::NotEquals(ref value)) => format!("expires != '{}'", value),
                        Some(crate::refresh_token::v1::expires_filter::Type::Before(ref value)) => format!("expires < '{}'", value),
                        Some(crate::refresh_token::v1::expires_filter::Type::After(ref value)) => format!("expires > '{}'", value),
                        None => String::new(),
                    }
                }
                Some(crate::refresh_token::v1::refresh_token_filter_condition::Condition::RevokedFilter(filter)) => {
                    match filter.r#type {
                        Some(crate::refresh_token::v1::revoked_filter::Type::Equals(value)) => format!("revoked = {}", value),
                        Some(crate::refresh_token::v1::revoked_filter::Type::NotEquals(value)) => format!("revoked != {}", value),

                        None => String::new(),
                    }
                }
                Some(crate::refresh_token::v1::refresh_token_filter_condition::Condition::TokenFilter(filter)) => {
                    match filter.r#type {
                        Some(crate::refresh_token::v1::token_filter::Type::Equals(ref value)) => format!("token = '{}'", value),
                        Some(crate::refresh_token::v1::token_filter::Type::NotEquals(ref value)) => format!("token != '{}'", value),
                        Some(crate::refresh_token::v1::token_filter::Type::Contains(ref value)) => format!("token LIKE '%{}%'", value),
                        None => String::new(),
                    }
                }
                None => String::new(),
            })
            .collect::<Vec<String>>()
            .join(" AND ")
    }
}

impl<'r> FromRow<'_, PgRow> for RefreshTokenModel {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(RefreshTokenModel {
            id: row.try_get("id")?,
            account_id: row.try_get("account_id")?,
            issued_at: Some(prost_types::Timestamp {
                seconds: row.try_get::<DateTime<Utc>, _>("issued_at")?.timestamp(),
                nanos: row
                    .try_get::<DateTime<Utc>, _>("issued_at")?
                    .timestamp_subsec_nanos() as i32,
            }),
            expires: Some(prost_types::Timestamp {
                seconds: row.try_get::<DateTime<Utc>, _>("expires")?.timestamp(),
                nanos: row
                    .try_get::<DateTime<Utc>, _>("expires")?
                    .timestamp_subsec_nanos() as i32,
            }),
            revoked: row.try_get("revoked")?,
            revocation_time: row
                .try_get::<Option<DateTime<Utc>>, _>("revocation_time")?
                .map(|dt| prost_types::Timestamp {
                    seconds: dt.timestamp(),
                    nanos: dt.timestamp_subsec_nanos() as i32,
                }),
            token: row.try_get("token")?,
        })
    }
}

#[async_trait]
impl RefreshTokenService for PostgresRefreshTokenService {
    async fn insert(
        &self, request: Request<RefreshTokenInsertRequest>,
    ) -> Result<Response<RefreshTokensResponse>, Status> {
        let req = request.into_inner();
        let account_ids: Vec<i32> = req.tokens.iter().map(|token| token.account_id).collect();
        let expires: Vec<DateTime<Utc>> = req
            .tokens
            .iter()
            .map(|token| {
                sqlx::types::chrono::DateTime::from_timestamp(
                    token.expires.unwrap().seconds,
                    token.expires.unwrap().nanos as u32,
                )
                .unwrap()
            })
            .collect();
        let tokens: Vec<String> = req.tokens.iter().map(|token| token.token.clone()).collect();

        match sqlx::query_as::<_, RefreshTokenModel>(
            r#"
            INSERT INTO refresh_tokens (account_id, expires, token)
            SELECT * FROM UNNEST($1::int[], $2::timestamp[], $3::text[])
            RETURNING id, account_id, issued_at, expires, revoked, revocation_time, token
        "#,
        )
        .bind(&account_ids)
        .bind(&expires)
        .bind(&tokens)
        .fetch_all(self.pool.as_ref())
        .await
        {
            Ok(tokens) => Ok(Response::new(RefreshTokensResponse {
                tokens,
                status: Some(OperationStatus {
                    success: true,
                    message: "Tokens inserted successfully".to_string(),
                }),
            })),
            Err(e) => match e {
                sqlx::Error::Database(e) => {
                    Err(Status::invalid_argument(format!("Invalid input: {}", e)))
                }
                _ => Err(Status::internal(format!("Database error: {}", e))),
            },
        }
    }

    async fn update(
        &self, request: Request<RefreshTokenUpdateRequest>,
    ) -> Result<Response<RefreshTokensResponse>, Status> {
        let req = request.into_inner();
        if req.params.is_none() {
            return Err(Status::invalid_argument("No ids provided"));
        }

        let filters = PostgresRefreshTokenService::build_filter_query(&req.params.unwrap().filters);

        let mut updates = Vec::new();
        for token in req.tokens {
            let mut query = String::from("UPDATE refresh_tokens SET ");
            let mut update_fields = Vec::new();

            if let Some(expires) = token.expires {
                update_fields.push(format!(
                    "expires = '{}'",
                    chrono::NaiveDateTime::from_timestamp(expires.seconds, expires.nanos as u32)
                ));
            }
            if let Some(revoked) = token.revoked {
                update_fields.push(format!("revoked = {}", revoked));
            }
            if let Some(revocation_time) = token.revocation_time {
                update_fields.push(format!(
                    "revocation_time = '{}'",
                    chrono::NaiveDateTime::from_timestamp(
                        revocation_time.seconds,
                        revocation_time.nanos as u32
                    )
                ));
            }

            if update_fields.is_empty() {
                return Err(Status::invalid_argument("No fields to update"));
            }

            query.push_str(&update_fields.join(", "));
            query.push_str(&format!(" WHERE {}", filters));
            query.push_str(" RETURNING *");

            match sqlx::query_as::<_, RefreshTokenModel>(&query)
                .fetch_one(self.pool.as_ref())
                .await
            {
                Ok(token) => updates.push(token),
                Err(e) => match e {
                    sqlx::Error::RowNotFound => {
                        return Err(Status::not_found("Token not found"));
                    }
                    _ => return Err(Status::internal(format!("Database error: {}", e))),
                },
            }
        }

        Ok(Response::new(RefreshTokensResponse {
            tokens: updates,
            status: Some(OperationStatus {
                success: true,
                message: "Tokens updated successfully".to_string(),
            }),
        }))
    }

    async fn delete(
        &self, request: Request<RefreshTokenDeleteRequest>,
    ) -> Result<Response<RefreshTokensResponse>, Status> {
        let req = request.into_inner();
        if req.params.is_none() {
            return Err(Status::invalid_argument("No ids provided"));
        }
        let filters = PostgresRefreshTokenService::build_filter_query(&req.params.unwrap().filters);
        let query = format!("DELETE FROM refresh_tokens WHERE {};", filters);

        match sqlx::query(&query).execute(self.pool.as_ref()).await {
            Ok(_) => Ok(Response::new(RefreshTokensResponse {
                tokens: Vec::new(),
                status: Some(OperationStatus {
                    success: true,
                    message: "Token deleted successfully".to_string(),
                }),
            })),
            Err(e) => Err(Status::internal(format!("Database error: {}", e))),
        }
    }

    async fn select(
        &self, request: Request<RefreshTokenSelectRequest>,
    ) -> Result<Response<RefreshTokensResponse>, Status> {
        let req = request.into_inner();
        let filters = PostgresRefreshTokenService::build_filter_query(&req.filters);

        let query = format!(
            "SELECT * FROM refresh_tokens WHERE {} LIMIT {} OFFSET {};",
            filters, req.limit, req.offset
        );

        match sqlx::query_as::<_, RefreshTokenModel>(&query)
            .fetch_all(self.pool.as_ref())
            .await
        {
            Ok(tokens) => Ok(Response::new(RefreshTokensResponse {
                tokens,
                status: Some(OperationStatus {
                    success: true,
                    message: "Tokens retrieved successfully".to_string(),
                }),
            })),
            Err(e) => Err(Status::internal(format!("Database error: {}", e))),
        }
    }
}
