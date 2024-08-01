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

  # Insert a test token for the update test
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
        "token_filter": {"equals": "test-token-updated"}
      }]
    }
  }' localhost:50051 db.refresh_token.v1.RefreshTokenService/Delete >/dev/null 2>&1
}

@test "Update Refresh Token" {
  result=$(grpcurl -plaintext -d '{
    "tokens": [{
      "expires": {"seconds": 1672531200},
      "revoked": true,
      "revocation_time": {"seconds": 1672531199},
      "token": "test-token-updated"
    }],
    "params": {
      "filters": [{
        "token_filter": {"equals": "test-token"}
      }]
    }
  }' localhost:50051 db.refresh_token.v1.RefreshTokenService/Update)

  echo "$result" | grep -q '"success": true'
  [ $? -eq 0 ]
}
