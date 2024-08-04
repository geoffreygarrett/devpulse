use async_trait::async_trait;
use tonic::{Request, Response, Status};

use crate::DataAccess;
pub use crate::proto::db_auth_v1::{
    AccountModel,
    account_service_server::{AccountService, AccountServiceServer}, AccountCreate, AccountDelete, AccountListResponse, AccountResponse, AccountsResponse,
    AccountUpdate, OperationStatus,
};

pub struct AccountServiceImpl<D>
where
    D: DataAccess<AccountModel, AccountCreate, AccountUpdate> + Sync + Send + 'static,
{
    pub data_accessor: D,
}

#[async_trait]
impl<D> AccountService for AccountServiceImpl<D>
where
    D: DataAccess<AccountModel, AccountCreate, AccountUpdate> + Sync + Send + 'static,
    Status: From<<D as DataAccess<AccountModel, AccountCreate, AccountUpdate>>::Error>,
{
    async fn insert(
        &self, request: Request<AccountCreate>,
    ) -> Result<Response<AccountResponse>, Status> {
        let account_create = request.into_inner();
        // self.data_accessor.create(&account_create).await.into()
        match self.data_accessor.create(&account_create).await {
            Ok(result) => Ok(Response::new(AccountResponse {
                account: Some(result),
                status: Some(OperationStatus {
                    success: true,
                    message: "Account created successfully".into(),
                }),
            })),
            Err(e) => Err(Status::internal(format!("Failed to create account: {}", e))),
            // Err(e) => Err(Status::internal(format!("Failed to create account: {}", e.to_string()))),
            // }
        }
    }

    async fn update(
        &self, request: Request<AccountUpdate>,
    ) -> Result<Response<AccountResponse>, Status> {
        let account_update = request.into_inner();
        match self.data_accessor.update(&account_update).await {
            Ok(account) => Ok(Response::new(AccountResponse {
                account: Some(account),
                status: Some(OperationStatus {
                    success: true,
                    message: "Account updated successfully".into(),
                }),
            })),
            Err(e) => Err(Status::internal(format!("Failed to update account: {}", e))),
        }
        // Err(e) => Err(Status::internal(format!("Failed to update account: {}", e))),
    }

    async fn delete(
        &self, request: Request<AccountDelete>,
    ) -> Result<Response<OperationStatus>, Status> {
        let id = request.into_inner().id;
        match self.data_accessor.delete(id as u64).await {
            Ok(_) => Ok(Response::new(OperationStatus {
                success: true,
                message: "Account deleted successfully".into(),
            })),
            Err(e) => Err(Status::internal(format!("Failed to delete account: {}", e))),
        }
    }

    async fn list(&self, _request: Request<()>) -> Result<Response<AccountListResponse>, Status> {
        match self.data_accessor.list().await {
            Ok(accounts) => Ok(Response::new(AccountListResponse {
                accounts,
                status: Some(OperationStatus {
                    success: true,
                    message: "Accounts listed successfully".into(),
                }),
            })),
            Err(e) => Err(Status::internal(format!("Failed to list accounts: {}", e))),
        }
    }
}

// impl AccountCreate {
//     pub fn sql_insert(&self) -> String {
//         let mut fields = Vec::new();
//         let mut values = Vec::new();
//         let mut params = Vec::new();
//         if !self.uuid.is_empty() {
//             fields.push("uuid");
//             values.push("$1");
//             params.push(&self.uuid as &dyn ::sqlx::Type);
//         }
//         if !self.given_name.is_empty() {
//             fields.push("given_name");
//             values.push("$2");
//             params.push(&self.given_name as &dyn sqlx::Type);
//         }
//         if !self.email.is_empty() {
//             fields.push("email");
//             values.push("$3");
//             params.push(&self.email as &dyn sqlx::Type);
//         }
//         if !self.hash.is_empty() {
//             fields.push("hash");
//             values.push("$4");
//             params.push(&self.hash as &dyn sqlx::Type);
//         }
//         if !self.avatar_url.is_empty() {
//             fields.push("avatar_url");
//             values.push("$5");
//             params.push(&self.avatar_url as &dyn sqlx::Type);
//         }
//         format!(
//             "INSERT INTO {}.{} ({}) VALUES ({})",
//             "public",
//             "accountcreate",
//             fields.join(", "),
//             values.join(", ")
//         )
//     }
// }

// impl AccountCreate {
//     pub fn sql_insert(&self) -> String {
//         let mut fields = Vec::new();
//         let mut values = Vec::new();
//         let mut params = Vec::new();
//         if !self.uuid.is_empty() {
//             fields.push("uuid");
//             values.push("$1");
//             params.push(&self.uuid as &dyn ::sqlx::Type<dyn ::sqlx::postgres::PgExecutor>);
//         }
//         if !self.given_name.is_empty() {
//             fields.push("given_name");
//             values.push("$2");
//             params.push(&self.given_name as &dyn ::sqlx::Type<dyn ::sqlx::postgres::PgExecutor>);
//         }
//         if !self.email.is_empty() {
//             fields.push("email");
//             values.push("$3");
//             params.push(&self.email as &dyn ::sqlx::Type<dyn ::sqlx::postgres::PgExecutor>);
//         }
//         if !self.hash.is_empty() {
//             fields.push("hash");
//             values.push("$4");
//             params.push(&self.hash as &dyn ::sqlx::Type<dyn ::sqlx::postgres::PgExecutor>);
//         }
//         if !self.avatar_url.is_empty() {
//             fields.push("avatar_url");
//             values.push("$5");
//             params.push(&self.avatar_url as &dyn ::sqlx::Type<dyn ::sqlx::postgres::PgExecutor>);
//         }
//         format!(
//             "INSERT INTO {}.{} ({}) VALUES ({})",
//             "public",
//             "accountcreate",
//             fields.join(", "),
//             values.join(", ")
//         )
//     }
// }
