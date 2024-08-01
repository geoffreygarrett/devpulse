#!/usr/bin/env bats

setup() {
  # Ensure the test environment is clean by deleting any existing test tokens
  grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "token_filter": {"equals": "test-token-1"}
      }]
    }
  }' localhost:50051 db.refresh_token.v1.RefreshTokenService/Delete >/dev/null 2>&1

  grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "token_filter": {"equals": "test-token-2"}
      }]
    }
  }' localhost:50051 db.refresh_token.v1.RefreshTokenService/Delete >/dev/null 2>&1

  # Insert test tokens for the select tests
  grpcurl -plaintext -d '{
    "tokens": [
      {
        "account_id": 1,
        "expires": {"seconds": 1672531199},
        "revoked": false,
        "token": "test-token-1"
      },
      {
        "account_id": 2,
        "expires": {"seconds": 1672531199},
        "revoked": false,
        "token": "test-token-2"
      }
    ]
  }' localhost:50051 db.refresh_token.v1.RefreshTokenService/Insert >/dev/null 2>&1
}

teardown() {
  # Clean up - Delete the test tokens
  grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "token_filter": {"equals": "test-token-1"}
      },
      {
        "token_filter": {"equals": "test-token-2"}
      }]
    }
  }' localhost:50051 db.refresh_token.v1.RefreshTokenService/Delete >/dev/null 2>&1
}

@test "Select Refresh Token with composite filter" {
  result=$(grpcurl -plaintext -d '{
    "filters": [{
      "account_id_filter": {"equals": 1}
    }, {
      "token_filter": {"equals": "test-token-1"}
    }],
    "limit": 10,
    "offset": 0
  }' localhost:50051 db.refresh_token.v1.RefreshTokenService/Select)

  echo "$result" | grep -q '"success": true'
  [ $? -eq 0 ]
}
