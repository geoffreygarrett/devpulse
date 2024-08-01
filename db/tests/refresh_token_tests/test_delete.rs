use prost_types::Timestamp;
use tonic::Request;
use tonic::transport::Channel;

use devpulse_db::account::v1::{
    account_service_client::AccountServiceClient, AccountDeleteRequest, AccountFilterCondition,
    AccountInsert, AccountInsertRequest, AccountsResponse,
};
use devpulse_db::refresh_token::v1::{
    refresh_token_filter_condition, refresh_token_service_client::RefreshTokenServiceClient,
    RefreshTokenDeleteRequest, RefreshTokenFilterCondition, RefreshTokenInsertRequest,
    RefreshTokenSelectRequest, RefreshTokensResponse, SharedInput, token_filter,
    TokenFilter,
};
use devpulse_db::refresh_token::v1::refresh_token_filter_condition::Condition::TokenFilter;

#[tokio::test]
async fn test_delete_refresh_token() -> Result<(), Box<dyn std::error::Error>> {
    // Setup gRPC clients
    let account_channel = Channel::from_static("http://localhost:50051")
        .connect()
        .await?;
    let mut account_client = AccountServiceClient::new(account_channel);

    let refresh_token_channel = Channel::from_static("http://localhost:50051")
        .connect()
        .await?;
    let mut refresh_token_client = RefreshTokenServiceClient::new(refresh_token_channel);

    // Step 1: Create an account for testing
    let account_insert_request = AccountInsertRequest {
        accounts: vec![AccountInsert {
            uuid: Some("123e4567-e89b-12d3-a456-426614174000".to_string()),
            given_name: Some("John".to_string()),
            email: Some("john.doe@example.com".to_string()),
            hash: Some("hashedpassword".to_string()),
            avatar_url: Some("http://example.com/avatar.jpg".to_string()),
        }],
        params: None,
    };

    let account_response: AccountsResponse = account_client
        .insert(Request::new(account_insert_request))
        .await?
        .into_inner();

    if !account_response.status.as_ref().unwrap().success {
        panic!("Failed to create test account: {:?}", account_response.status);
    }

    let account_id = account_response.accounts[0].id;

    // Step 2: Ensure the test environment is clean by deleting any existing test tokens
    let delete_token_request = RefreshTokenDeleteRequest {
        params: Some(SharedInput {
            filters: vec![RefreshTokenFilterCondition {
                condition: Some(refresh_token_filter_condition::Condition::TokenFilter(
                    TokenFilter {
                        r#type: TokenFilter(token_filter::Type::Equals("test-token".to_string())),
                    },
                )),
            }],
            return_data: false,
        }),
    };

    let delete_token_response: RefreshTokensResponse = refresh_token_client
        .delete(Request::new(delete_token_request))
        .await?
        .into_inner();

    if !delete_token_response.status.as_ref().unwrap().success {
        panic!("Failed to delete existing test tokens: {:?}", delete_token_response.status);
    }

    // Step 3: Insert a test token for the delete test
    let expires = Timestamp {
        seconds: 1735689599, // 2025-12-31T23:59:59Z
        nanos: 0,
    };

    let insert_token_request = RefreshTokenInsertRequest {
        tokens: vec![RefreshTokenModel {
            account_id,
            expires: Some(expires),
            token: "test-token".to_string(),
        }],
    };

    let insert_token_response: RefreshTokensResponse = refresh_token_client
        .insert(Request::new(insert_token_request))
        .await?
        .into_inner();

    if !insert_token_response.status.as_ref().unwrap().success {
        panic!("Failed to insert test token: {:?}", insert_token_response.status);
    }

    // Step 4: Perform the delete operation
    let delete_token_request = RefreshTokenDeleteRequest {
        params: Some(SharedInput {
            filters: vec![RefreshTokenFilterCondition {
                condition: Some(refresh_token_filter_condition::Condition::TokenFilter(
                    token_filter {
                        equals: "test-token".to_string(),
                    },
                )),
            }],
            return_data: false,
        }),
    };

    let delete_token_response: RefreshTokensResponse = refresh_token_client
        .delete(Request::new(delete_token_request))
        .await?
        .into_inner();

    if !delete_token_response.status.as_ref().unwrap().success {
        panic!("Failed to delete refresh token: {:?}", delete_token_response.status);
    }

    // Step 5: Verify the token is actually deleted
    let verify_token_request = RefreshTokenSelectRequest {
        filters: vec![RefreshTokenFilterCondition {
            condition: Some(refresh_token_filter_condition::Condition::TokenFilter(token_filter {
                equals: "test-token".to_string(),
            })),
        }],
        limit: 10,
        offset: 0,
        count: false,
        order_by: "".to_string(),
        order: "".to_string(),
    };

    let verify_token_response: RefreshTokensResponse = refresh_token_client
        .select(Request::new(verify_token_request))
        .await?
        .into_inner();

    if !verify_token_response.status.as_ref().unwrap().success {
        panic!("Failed to verify refresh token deletion: {:?}", verify_token_response.status);
    }

    if !verify_token_response.tokens.is_empty() {
        panic!("Token still exists after deletion: {:?}", verify_token_response.tokens);
    }

    // Clean up - Delete the test account
    let delete_account_request = AccountDeleteRequest {
        params: Some(SharedInput {
            filters: vec![AccountFilterCondition {
                condition: Some(account_filter_condition::Condition::UuidFilter(uuid_filter {
                    equals: "123e4567-e89b-12d3-a456-426614174000".to_string(),
                })),
            }],
            return_data: false,
        }),
    };

    let delete_account_response: AccountsResponse = account_client
        .delete(Request::new(delete_account_request))
        .await?
        .into_inner();

    if !delete_account_response.status.as_ref().unwrap().success {
        panic!("Failed to delete test account: {:?}", delete_account_response.status);
    }

    Ok(())
}
