#!/usr/bin/env bats

setup() {
  # Ensure the test environment is clean by deleting any existing test accounts and tokens
  delete_account_result=$(grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "uuid_filter": {"equals": "123e4567-e89b-12d3-a456-426614174000"}
      }]
    }
  }' localhost:50051 db.account.v1.AccountService/Delete)
  echo "Account deletion result: $delete_account_result"

  delete_token_result=$(grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "token_filter": {"equals": "test-token"}
      }]
    }
  }' localhost:50051 db.refresh_token.v1.RefreshTokenService/Delete)
  echo "Token deletion result: $delete_token_result"

  # Create an account for testing
  account_result=$(grpcurl -plaintext -d '{
    "accounts": [{
      "uuid": "123e4567-e89b-12d3-a456-426614174000",
      "given_name": "John",
      "email": "john.doe@example.com",
      "hash": "hashedpassword",
      "avatar_url": "http://example.com/avatar.jpg"
    }]
  }' localhost:50051 db.account.v1.AccountService/Insert)

  if [ $? -ne 0 ]; then
    echo "Failed to create test account: $account_result"
    exit 1
  fi

  # Extract account id from the result
  account_id=$(echo "$account_result" | grep -o '"id": [0-9]*' | grep -o '[0-9]*')
  if [ -z "$account_id" ]; then
    echo "Failed to extract account_id from result: $account_result"
    exit 1
  fi

  # Insert a test token for the delete test
  insert_result=$(grpcurl -plaintext -d "{
    \"tokens\": [{
      \"account_id\": $account_id,
      \"expires\": \"2025-12-31T23:59:59Z\",
      \"token\": \"test-token\"
    }]
  }" localhost:50051 db.refresh_token.v1.RefreshTokenService/Insert)

  if [ $? -ne 0 ]; then
    echo "Failed to insert test token: $insert_result"
    exit 1
  fi
}

teardown() {
  # Clean up - Delete the test token
  delete_result=$(grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "token_filter": {"equals": "test-token"}
      }]
    }
  }' localhost:50051 db.refresh_token.v1.RefreshTokenService/Delete)

  if [ $? -ne 0 ]; then
    echo "Failed to delete test tokens during teardown: $delete_result"
    exit 1
  fi

  # Clean up - Delete the test account
  delete_account_result=$(grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "uuid_filter": {"equals": "123e4567-e89b-12d3-a456-426614174000"}
      }]
    }
  }' localhost:50051 db.account.v1.AccountService/Delete)

  if [ $? -ne 0 ]; then
    echo "Failed to delete test account during teardown: $delete_account_result"
    exit 1
  fi
}

@test "Delete Refresh Token" {
  # Perform the delete operation
  result=$(grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "token_filter": {"equals": "test-token"}
      }]
    }
  }' localhost:50051 db.refresh_token.v1.RefreshTokenService/Delete)

  if [ $? -ne 0 ]; then
    echo "Failed to delete refresh token: $result"
    exit 1
  fi

  echo "$result" | grep -q '"success": true'
  [ $? -eq 0 ]

  # Verify the token is actually deleted
  verify_result=$(grpcurl -plaintext -d '{
    "filters": [{
      "token_filter": {"equals": "test-token"}
    }],
    "limit": 10,
    "offset": 0
  }' localhost:50051 db.refresh_token.v1.RefreshTokenService/Select)

  if [ $? -ne 0 ]; then
    echo "Failed to verify refresh token deletion: $verify_result"
    exit 1
  fi

  if echo "$verify_result" | grep -q '"success": true' && echo "$verify_result" | grep -q '"tokens": \[\]'; then
    echo "Token successfully deleted."
  else
    echo "Token still exists after deletion."
    exit 1
  fi
}
