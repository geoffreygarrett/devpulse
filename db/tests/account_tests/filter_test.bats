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

  grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "uuid_filter": {"equals": "223e4567-e89b-12d3-a456-426614174001"}
      }]
    }
  }' localhost:50051 db.account.v1.AccountService/Delete >/dev/null

  grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "email_filter": {"equals": "john.doe@example.com"}
      }]
    }
  }' localhost:50051 db.account.v1.AccountService/Delete >/dev/null

  grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "email_filter": {"equals": "jane.doe@example.com"}
      }]
    }
  }' localhost:50051 db.account.v1.AccountService/Delete >/dev/null

  # Insert test accounts for the select tests
  grpcurl -plaintext -d '{
    "accounts": [
      {
        "uuid": "123e4567-e89b-12d3-a456-426614174000",
        "given_name": "John",
        "email": "john.doe@example.com",
        "hash": "hashedpassword",
        "avatar_url": "https://example.com/avatar.jpg"
      },
      {
        "uuid": "223e4567-e89b-12d3-a456-426614174001",
        "given_name": "Jane",
        "email": "jane.doe@example.com",
        "hash": "anotherhashedpassword",
        "avatar_url": "https://example.com/avatar2.jpg"
      }
    ]
  }' localhost:50051 db.account.v1.AccountService/Insert >/dev/null
}

teardown() {
  # Clean up - Delete the test accounts
  grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "uuid_filter": {"equals": "123e4567-e89b-12d3-a456-426614174000"}
      },
      {
        "uuid_filter": {"equals": "223e4567-e89b-12d3-a456-426614174001"}
      }]
    }
  }' localhost:50051 db.account.v1.AccountService/Delete >/dev/null
}

@test "Select Account by email" {
  result=$(grpcurl -plaintext -d '{
    "filters": [{
      "email_filter": {"equals": "john.doe@example.com"}
    }],
    "limit": 10,
    "offset": 0
  }' localhost:50051 db.account.v1.AccountService/Select)

  echo "$result" | grep -q '"success": true'
  [ $? -eq 0 ]
}

@test "Select Account by given name like" {
  result=$(grpcurl -plaintext -d '{
    "filters": [{
      "given_name_filter": {"like": "Jo"}
    }],
    "limit": 10,
    "offset": 0
  }' localhost:50051 db.account.v1.AccountService/Select)

  echo "$result" | grep -q '"success": true'
  [ $? -eq 0 ]
}

@test "Select Account by hash not equals" {
  result=$(grpcurl -plaintext -d '{
    "filters": [{
      "hash_filter": {"not_equals": "hashedpassword"}
    }],
    "limit": 10,
    "offset": 0
  }' localhost:50051 db.account.v1.AccountService/Select)

  echo "$result" | grep -q '"success": true'
  [ $? -eq 0 ]
}

@test "Select Account by avatar URL" {
  result=$(grpcurl -plaintext -d '{
    "filters": [{
      "avatar_url_filter": {"equals": "https://example.com/avatar.jpg"}
    }],
    "limit": 10,
    "offset": 0
  }' localhost:50051 db.account.v1.AccountService/Select)

  echo "$result" | grep -q '"success": true'
  [ $? -eq 0 ]
}

@test "Select Account by created at range" {
  result=$(grpcurl -plaintext -d '{
    "filters": [{
      "created_at_filter": {"after": "2021-01-01T00:00:00Z", "before": "2023-12-31T23:59:59Z"}
    }],
    "limit": 10,
    "offset": 0
  }' localhost:50051 db.account.v1.AccountService/Select)

  echo "$result" | grep -q '"success": true'
  [ $? -eq 0 ]
}

@test "Select Account with composite filter" {
  result=$(grpcurl -plaintext -d '{
    "filters": [{
      "given_name_filter": {"equals": "John"}
    }, {
      "email_filter": {"equals": "john.doe@example.com"}
    }],
    "limit": 10,
    "offset": 0
  }' localhost:50051 db.account.v1.AccountService/Select)

  echo "$result" | grep -q '"success": true'
  [ $? -eq 0 ]
}
