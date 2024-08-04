#!/usr/bin/env bats

setup() {
  # Ensure the test environment is clean by deleting the test account if it exists
  grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "uuid_filter": {"equals": "00000000-0000-0000-0000-000000000000"}
      }]
    }
  }' localhost:50051 db.account.v1.AccountService/Delete >/dev/null 2>&1

  # Insert a test account
  result=$(grpcurl -plaintext -d '{
    "accounts": [{
      "uuid": "00000000-0000-0000-0000-000000000000",
      "given_name": "John",
      "email": "john.doe@example.com",
      "hash": "hashedpassword",
      "avatar_url": "http://example.com/avatar.jpg"
    }]
  }' localhost:50051 db.account.v1.AccountService/Insert)
  echo "$result"
  echo "$result" | grep -q '"success": true'
  [ $? -eq 0 ]
}

teardown() {
  # Clean up - Delete the test account
  grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "uuid_filter": {"equals": "00000000-0000-0000-0000-000000000000"}
      }]
    }
  }' localhost:50051 db.account.v1.AccountService/Delete >/dev/null 2>&1
}

@test "Delete Account" {
  # Perform the delete operation
  result=$(grpcurl -plaintext -d '{
    "params": {
      "filters": [{
        "uuid_filter": {"equals": "00000000-0000-0000-0000-000000000000"}
      }]
    }
  }' localhost:50051 db.account.v1.AccountService/Delete)

  echo "$result"
  echo "$result" | grep -q '"success": true'
  [ $? -eq 0 ]

  # Verify the account is actually deleted
  verify_result=$(grpcurl -plaintext -d '{
    "filters": [{
      "uuid_filter": {"equals": "00000000-0000-0000-0000-000000000000"}
    }],
    "limit": 10,
    "offset": 0
  }' localhost:50051 db.account.v1.AccountService/Select)

  if echo "$verify_result" | grep -q '"success": true'; then
    echo "Account successfully deleted."
  else
    echo "Account still exists after deletion."
    exit 1
  fi
}
