#!/usr/bin/env bats

setup() {
  # Create Account for testing
  result=$(grpcurl -plaintext -d '{
    "accounts": [{
      "uuid": "123e4567-e89b-12d3-a456-426614174000",
      "given_name": "John",
      "email": "john.doe@example.com",
      "hash": "hashedpassword",
      "avatar_url":"https://example.com/avatar.jpg"
    }]
  }' localhost:50051 db.account.v1.AccountService/Insert)

  echo "$result" | grep -q '"success": true'
  [ $? -eq 0 ]
}

teardown() {
  # Delete Account after testing
  result=$(grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "uuid_filter": {"equals": "123e4567-e89b-12d3-a456-426614174000"}
      }]
    }
  }' localhost:50051 db.account.v1.AccountService/Delete)

  echo "$result" | grep -q '"success": true'
  [ $? -eq 0 ]
}

@test "Update Account" {
  result=$(grpcurl -plaintext -d '{
    "accounts": [{
      "given_name": "John Updated",
      "email": "john.updated@example.com",
      "hash": "updatedhashedpassword",
      "avatar_url": "https://example.com/avatar-updated.jpg"
    }],
    "params": {
      "filters": [{
        "uuid_filter": {"equals": "123e4567-e89b-12d3-a456-426614174000"}
      }]
    }
  }' localhost:50051 db.account.v1.AccountService/Update)

  echo "$result" | grep -q '"success": true'
  [ $? -eq 0 ]
}

@test "Select Account" {
  result=$(grpcurl -plaintext -d '{
    "filters": [{
      "uuid_filter": {"equals": "123e4567-e89b-12d3-a456-426614174000"}
    }],
    "limit": 1,
    "offset": 0
  }' localhost:50051 db.account.v1.AccountService/Select)

  echo "$result" | grep -q '"uuid": "123e4567-e89b-12d3-a456-426614174000"'
  [ $? -eq 0 ]
}
