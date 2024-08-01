#!/usr/bin/env bats

setup() {
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

  echo "$account_result" | grep -q '"success": true'
  [ $? -eq 0 ]

  # Extract the account ID from the account creation response
  account_id=$(echo "$account_result" | jq -r '.accounts[0].id')

  # Ensure the test environment is clean by deleting any existing test tokens
  grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "token_filter": {"equals": "test-token"}
      }]
    }
  }' localhost:50051 db.refresh_token.v1.RefreshTokenService/Delete >/dev/null 2>&1
}

teardown() {
  # Clean up - Delete the test token
  grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "token_filter": {"equals": "test-token"}
      }]
    }
  }' localhost:50051 db.refresh_token.v1.RefreshTokenService/Delete >/dev/null 2>&1

  # Clean up - Delete the test account
  grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "uuid_filter": {"equals": "123e4567-e89b-12d3-a456-426614174000"}
      }]
    }
  }' localhost:50051 db.account.v1.AccountService/Delete >/dev/null 2>&1
}

@test "Insert Refresh Token" {
  result=$(grpcurl -plaintext -d '{
    "tokens": [{
      "account_id": '"$account_id"',
      "expires": "2023-12-31T23:59:59Z",
      "token": "test-token"
    }]
  }' localhost:50051 db.refresh_token.v1.RefreshTokenService/Insert)

  echo "$result"
  echo "$result" | grep -q '"success": true'
  [ $? -eq 0 ]
}
