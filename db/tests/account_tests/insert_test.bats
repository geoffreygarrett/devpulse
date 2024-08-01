#!/usr/bin/env bats

setup() {
  # Ensure the account to be tested doesn't exist before the test
  grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "uuid_filter": {"equals": "123e4567-e89b-12d3-a456-426614174000"}
      }]
    }
  }' localhost:50051 db.account.v1.AccountService/Delete > /dev/null
}

teardown() {
  # Clean up - Delete the test account
  grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "uuid_filter": {"equals": "123e4567-e89b-12d3-a456-426614174000"}
      }]
    }
  }' localhost:50051 db.account.v1.AccountService/Delete > /dev/null
}

@test "Insert Account" {
  result=$(grpcurl -plaintext -d '{
    "accounts": [{
      "uuid": "123e4567-e89b-12d3-a456-426614174000",
      "given_name": "John",
      "email": "john.doe@example.com",
      "hash": "hashedpassword",
      "avatar_url": "https://example.com/avatar.jpg"
    }]
  }' localhost:50051 db.account.v1.AccountService/Insert)

  echo "$result" | grep -q '"success": true'
  [ $? -eq 0 ]
}
