use std::str::FromStr;
use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{FromRow, PgPool, Row};
use tonic::{Request, Response, Status};

use crate::account::v1::{
    account_filter_condition, account_service_server::AccountService, AccountDeleteRequest,
    AccountFilterCondition, AccountInsertRequest, AccountModel, AccountSelectRequest, AccountsResponse, AccountUpdateRequest,
    avatar_url_filter, created_at_filter, email_filter, given_name_filter,
    hash_filter, id_filter, OperationStatus, uuid_filter,
};

pub struct PostgresAccountService {
    pub pool: Arc<PgPool>,
}

impl PostgresAccountService {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            pool: Arc::new(pool.clone()),
        }
    }
    fn build_filter_query(filters: &[AccountFilterCondition]) -> String {
        filters
            .iter()
            .map(|filter| match &filter.condition {
                Some(account_filter_condition::Condition::IdFilter(filter)) => {
                    match filter.r#type {
                        Some(id_filter::Type::Equals(value)) => format!("id = {}", value),
                        Some(id_filter::Type::NotEquals(value)) => format!("id != {}", value),
                        Some(id_filter::Type::GreaterThan(value)) => format!("id > {}", value),
                        Some(id_filter::Type::LessThan(value)) => format!("id < {}", value),
                        None => String::new(),
                    }
                }
                Some(account_filter_condition::Condition::UuidFilter(filter)) => {
                    match filter.r#type {
                        Some(uuid_filter::Type::Equals(ref value)) => format!("uuid = '{}'", value),
                        Some(uuid_filter::Type::NotEquals(ref value)) => {
                            format!("uuid != '{}'", value)
                        }
                        None => String::new(),
                    }
                }
                Some(account_filter_condition::Condition::GivenNameFilter(filter)) => {
                    match filter.r#type {
                        Some(given_name_filter::Type::Equals(ref value)) => {
                            format!("given_name = '{}'", value)
                        }
                        Some(given_name_filter::Type::NotEquals(ref value)) => {
                            format!("given_name != '{}'", value)
                        }
                        Some(given_name_filter::Type::Like(ref value)) => {
                            format!("given_name LIKE '%{}%'", value)
                        }
                        None => String::new(),
                    }
                }
                Some(account_filter_condition::Condition::EmailFilter(filter)) => match filter
                    .r#type
                {
                    Some(email_filter::Type::Equals(ref value)) => format!("email = '{}'", value),
                    Some(email_filter::Type::NotEquals(ref value)) => {
                        format!("email != '{}'", value)
                    }
                    Some(email_filter::Type::Like(ref value)) => {
                        format!("email LIKE '%{}%'", value)
                    }
                    None => String::new(),
                },
                Some(account_filter_condition::Condition::HashFilter(filter)) => {
                    match filter.r#type {
                        Some(hash_filter::Type::Equals(ref value)) => format!("hash = '{}'", value),
                        Some(hash_filter::Type::NotEquals(ref value)) => {
                            format!("hash != '{}'", value)
                        }
                        None => String::new(),
                    }
                }
                Some(account_filter_condition::Condition::AvatarUrlFilter(filter)) => {
                    match filter.r#type {
                        Some(avatar_url_filter::Type::Equals(ref value)) => {
                            format!("avatar_url = '{}'", value)
                        }
                        Some(avatar_url_filter::Type::NotEquals(ref value)) => {
                            format!("avatar_url != '{}'", value)
                        }
                        None => String::new(),
                    }
                }
                Some(account_filter_condition::Condition::CreatedAtFilter(filter)) => {
                    match filter.r#type {
                        Some(created_at_filter::Type::Equals(ref value)) => {
                            format!("created_at = '{}'", value)
                        }
                        Some(created_at_filter::Type::NotEquals(ref value)) => {
                            format!("created_at != '{}'", value)
                        }
                        Some(created_at_filter::Type::Before(ref value)) => {
                            format!("created_at < '{}'", value)
                        }
                        Some(created_at_filter::Type::After(ref value)) => {
                            format!("created_at > '{}'", value)
                        }
                        None => String::new(),
                    }
                }
                None => String::new(),
            })
            .collect::<Vec<String>>()
            .join(" AND ")
    }
}

// impl FromRow<'_, PgRow> for AccountModel {
//     fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
//         Ok(AccountModel {
//             id: row.try_get("id")?,
//             uuid: row.try_get::<Uuid, _>("uuid")?.to_string(),
//             given_name: row.try_get("given_name")?,
//             email: row.try_get("email")?,
//             hash: row.try_get("hash")?,
//             avatar_url: row.try_get("avatar_url")?,
//             created_at: row
//                 .try_get::<chrono::DateTime<chrono::Utc>, _>("created_at")?
//                 .to_string(),
//         })
//     }
// }

impl FromRow<'_, sqlx::postgres::PgRow> for AccountModel {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(AccountModel {
            id: row.try_get::<i32, _>("id")?,
            uuid: row.try_get::<sqlx::types::Uuid, _>("uuid")?.to_string(),
            given_name: Some(
                row.try_get::<Option<String>, _>("given_name")?
                    .unwrap_or_default(),
            ),
            email: row.try_get::<String, _>("email")?,
            hash: Some(row.try_get::<String, _>("hash")?),
            avatar_url: row.try_get::<Option<String>, _>("avatar_url")?,
            created_at: row
                .try_get::<chrono::DateTime<chrono::Utc>, _>("created_at")?
                .to_string(),
        })
    }
}

#[async_trait]
impl AccountService for PostgresAccountService {
    async fn insert(
        &self, request: Request<AccountInsertRequest>,
    ) -> Result<Response<AccountsResponse>, Status> {
        let req = request.into_inner();
        let uuids: Vec<Option<sqlx::types::Uuid>> = req
            .accounts
            .iter()
            .map(|account| {
                account
                    .uuid
                    .clone()
                    .map(|x| sqlx::types::Uuid::from_str(&x).unwrap())
            })
            .collect();
        let given_names: Vec<Option<String>> = req
            .accounts
            .iter()
            .map(|account| account.given_name.clone())
            .collect();
        let emails: Vec<Option<String>> = req
            .accounts
            .iter()
            .map(|account| account.email.clone())
            .collect();
        let hashes: Vec<Option<String>> = req
            .accounts
            .iter()
            .map(|account| account.hash.clone())
            .collect();
        let avatar_urls: Vec<Option<String>> = req
            .accounts
            .iter()
            .map(|account| account.avatar_url.clone())
            .collect();

        println!("uuids: {:?}", uuids);
        println!("given_names: {:?}", given_names);
        println!("emails: {:?}", emails);
        println!("hashes: {:?}", hashes);
        println!("avatar_urls: {:?}", avatar_urls);

        match sqlx::query_as::<_, AccountModel>(
            r#"
            INSERT INTO accounts (uuid, given_name, email, hash, avatar_url)
            SELECT * FROM UNNEST($1::uuid[], $2::text[], $3::text[], $4::text[], $5::text[])
            RETURNING id, uuid, given_name, email, hash, avatar_url, created_at
        "#,
        )
        .bind(&uuids as &[Option<sqlx::types::Uuid>])
        .bind(&given_names as &[Option<String>])
        .bind(&emails as &[Option<String>])
        .bind(&hashes as &[Option<String>])
        .bind(&avatar_urls as &[Option<String>])
        .fetch_all(self.pool.as_ref())
        .await
        {
            Ok(accounts) => Ok(Response::new(AccountsResponse {
                accounts,
                status: Some(OperationStatus {
                    success: true,
                    message: "Accounts inserted successfully".to_string(),
                }),
            })),
            Err(e) => match e {
                sqlx::Error::Database(_) => {
                    return Err(Status::already_exists("Account already exists"));
                }
                _ => return Err(Status::internal(format!("Database error: {}", e))),
            },
        }
    }

    async fn update(
        &self, request: Request<AccountUpdateRequest>,
    ) -> Result<Response<AccountsResponse>, Status> {
        let req = request.into_inner();
        let filters = PostgresAccountService::build_filter_query(&req.params.unwrap().filters);

        let mut accounts = Vec::new();

        for account in req.accounts {
            let mut query = String::from("UPDATE accounts SET ");
            let mut updates = Vec::new();

            if let Some(given_name) = account.given_name {
                updates.push(format!("given_name = '{}'", given_name));
            }
            if let Some(email) = account.email {
                updates.push(format!("email = '{}'", email));
            }
            if let Some(hash) = account.hash {
                updates.push(format!("hash = '{}'", hash));
            }
            if let Some(avatar_url) = account.avatar_url {
                updates.push(format!("avatar_url = '{}'", avatar_url));
            }

            if updates.is_empty() {
                return Err(Status::invalid_argument("No fields to update"));
            }

            query.push_str(&updates.join(", "));
            query.push_str(&format!(" WHERE {}", filters));
            query.push_str(" RETURNING *");

            match sqlx::query_as::<_, AccountModel>(&query)
                .fetch_one(self.pool.as_ref())
                .await
            {
                Ok(account) => accounts.push(account),
                // Err(e) => return Err(Status::internal(format!("Database error: {}", e))),
                Err(e) => match e {
                    sqlx::Error::RowNotFound => {
                        return Err(Status::not_found("Account not found"));
                    }
                    _ => return Err(Status::internal(format!("Database error: {}", e))),
                },
            }
        }

        Ok(Response::new(AccountsResponse {
            accounts,
            status: Some(OperationStatus {
                success: true,
                message: "Accounts updated successfully".to_string(),
            }),
        }))
    }

    async fn delete(
        &self, request: Request<AccountDeleteRequest>,
    ) -> Result<Response<AccountsResponse>, Status> {
        let req = request.into_inner();
        let filters = PostgresAccountService::build_filter_query(&req.params.unwrap().filters);

        let query = format!("DELETE FROM accounts WHERE {};", filters);

        match sqlx::query(&query).execute(self.pool.as_ref()).await {
            Ok(_) => Ok(Response::new(AccountsResponse {
                accounts: Vec::new(),
                status: Some(OperationStatus {
                    success: true,
                    message: "Accounts deleted successfully".to_string(),
                }),
            })),
            Err(e) => Err(Status::internal(format!("Database error: {}", e))),
        }
    }

    async fn select(
        &self, request: Request<AccountSelectRequest>,
    ) -> Result<Response<AccountsResponse>, Status> {
        let req = request.into_inner();
        let filters = PostgresAccountService::build_filter_query(&req.filters);

        let query = format!(
            "SELECT * FROM accounts WHERE {} LIMIT {} OFFSET {};",
            filters, req.limit, req.offset
        );

        match sqlx::query_as::<_, AccountModel>(&query)
            .fetch_all(self.pool.as_ref())
            .await
        {
            Ok(accounts) => Ok(Response::new(AccountsResponse {
                accounts,
                status: Some(OperationStatus {
                    success: true,
                    message: "Accounts retrieved successfully".to_string(),
                }),
            })),
            Err(e) => Err(Status::internal(format!("Database error: {}", e))),
        }
    }
}
