#!/usr/bin/env bats

setup() {
  # Ensure the test environment is clean by deleting any existing test accounts
  grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "uuid_filter": {"equals": "123e4567-e89b-12d3-a456-426614174000"}
      }]
    }
  }' localhost:50051 db.account.v1.AccountService/Delete >/dev/null

  # Insert a test account for the select test
  grpcurl -plaintext -d '{
    "accounts": [{
      "uuid": "123e4567-e89b-12d3-a456-426614174000",
      "given_name": "John Updated",
      "email": "john.updated@example.com",
      "hash": "hashedpassword",
      "avatar_url": "https://example.com/avatar.jpg"
    }]
  }' localhost:50051 db.account.v1.AccountService/Insert >/dev/null
}

teardown() {
  # Clean up - Delete the test account
  grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "uuid_filter": {"equals": "123e4567-e89b-12d3-a456-426614174000"}
      }]
    }
  }' localhost:50051 db.account.v1.AccountService/Delete >/dev/null
}

@test "Select Account" {
  result=$(grpcurl -plaintext -d '{
    "filters": [{
      "email_filter": {"equals": "john.updated@example.com"}
    }],
    "limit": 10,
    "offset": 0
  }' localhost:50051 db.account.v1.AccountService/Select)

  echo "$result" | grep -q '"success": true'
  [ $? -eq 0 ]
}
