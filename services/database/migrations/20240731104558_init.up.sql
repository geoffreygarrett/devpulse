-- Migration script for creating tables for Account, Identity, and RefreshToken

-- Create the Account table
CREATE TABLE accounts
(
    id         SERIAL PRIMARY KEY,
    uuid       UUID         NOT NULL,
    given_name VARCHAR(255) NOT NULL,
    email      VARCHAR(255) NOT NULL UNIQUE,
    hash       VARCHAR(255) NOT NULL,
    avatar_url VARCHAR(255),
    created_at TIMESTAMP    NOT NULL DEFAULT NOW()
);

-- Create the Identity table
CREATE TABLE identities
(
    id         SERIAL PRIMARY KEY,
    account_id INT NOT NULL,
    source     INT NOT NULL,
    FOREIGN KEY (account_id) REFERENCES accounts (id)
);

-- Create the RefreshToken table
CREATE TABLE refresh_tokens
(
    id              SERIAL PRIMARY KEY,
    account_id      INT       NOT NULL,
    issued_at       TIMESTAMP NOT NULL DEFAULT NOW(),
    expires         TIMESTAMP NOT NULL,
    revoked         BOOLEAN   NOT NULL DEFAULT FALSE,
    revocation_time TIMESTAMP,
    token           TEXT      NOT NULL,
    FOREIGN KEY (account_id) REFERENCES accounts (id)
);

-- Insert sample data into IdentitySource enum
-- Note: PostgreSQL doesn't have built-in support for enums with strings like protobuf.
-- You can use a check constraint or a separate lookup table.
CREATE TABLE identity_sources
(
    value INT PRIMARY KEY,
    name  VARCHAR(255) NOT NULL
);

INSERT INTO identity_sources (value, name)
VALUES (0, 'PASSWORD'),
       (1, 'GOOGLE');
