#!/bin/bash

echo "Inserting an Account..."
grpcurl -plaintext -d '{
  "uuid": "123e4567-e89b-12d3-a456-426614174000",
  "given_name": "John",
  "email": "john.doe@exampssleds2.com",
  "hash": "somehashedpassword",
  "avatar_url": "http://example.com/avatar.jpg"
}' localhost:50051 db.auth.v1.AccountService/Insert

#  "uuid": {"value": "123e4567-e89b-12d3-a456-426614174000"},

#echo "Updating an Account..."
#sleep 2  # Sleep for sync reasons or handling response
#grpcurl -plaintext -d '{
#  "id": 1,
#  "uuid": {"value": "123e4567-e89b-12d3-a456-426614174000"},
#  "given_name": "John Updated",
#  "email": "john.updated@example.com",
#  "hash": "updatedhashedpassword",
#  "avatar_url": "http://example.com/new-avatar.jpg"
#}' localhost:50051 db.auth.v1.AccountService/Update
#
#echo "Deleting an Account..."
#sleep 2
#grpcurl -plaintext -d '{
#  "id": 1
#}' localhost:50051 db.auth.v1.AccountService/Delete
#
echo "Listing All Accounts..."
sleep 2
grpcurl -plaintext localhost:50051 db.auth.v1.AccountService/List
