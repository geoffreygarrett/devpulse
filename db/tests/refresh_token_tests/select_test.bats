#!/usr/bin/env bats

setup() {
  # Ensure the test environment is clean by deleting any existing test tokens
  grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "token_filter": {"equals": "test-token"}
      }]
    }
  }' localhost:50051 db.refresh_token.v1.RefreshTokenService/Delete >/dev/null 2>&1

  # Insert a test token for the select test
  grpcurl -plaintext -d '{
    "tokens": [{
      "account_id": 1,
      "expires": {"seconds": 1672531199},
      "revoked": false,
      "token": "test-token"
    }]
  }' localhost:50051 db.refresh_token.v1.RefreshTokenService/Insert >/dev/null 2>&1
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
}

@test "Select Refresh Token" {
  result=$(grpcurl -plaintext -d '{
    "filters": [{
      "token_filter": {"equals": "test-token"}
    }],
    "limit": 10,
    "offset": 0
  }' localhost:50051 db.refresh_token.v1.RefreshTokenService/Select)

  echo "$result" | grep -q '"success": true'
  [ $? -eq 0 ]
}
