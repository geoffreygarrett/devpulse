-- Create the users table
CREATE TABLE IF NOT EXISTS users
(
    instance_id                 UUID                                                                                   NULL,
    id                          UUID PRIMARY KEY                                                                                DEFAULT gen_random_uuid(),
    aud                         VARCHAR(255)                                                                           NULL,
    role                        VARCHAR(255)                                                                           NULL,
    email                       VARCHAR(255)                                                                           NOT NULL UNIQUE,
    encrypted_password          VARCHAR(255)                                                                           NOT NULL,
    email_confirmed_at          TIMESTAMPTZ                                                                            NULL,
    invited_at                  TIMESTAMPTZ                                                                            NULL,
    confirmation_token          VARCHAR(255)                                                                           NULL,
    confirmation_sent_at        TIMESTAMPTZ                                                                            NULL,
    recovery_token              VARCHAR(255)                                                                           NULL,
    recovery_sent_at            TIMESTAMPTZ                                                                            NULL,
    email_change_token_new      VARCHAR(255)                                                                           NULL,
    email_change                VARCHAR(255)                                                                           NULL,
    email_change_sent_at        TIMESTAMPTZ                                                                            NULL,
    last_sign_in_at             TIMESTAMPTZ                                                                            NULL,
    raw_app_meta_data           JSONB                                                                                  NULL,
    raw_user_meta_data          JSONB                                                                                  NULL,
    is_super_admin              BOOLEAN                                                                                NULL,
    created_at                  TIMESTAMPTZ                                                                                     DEFAULT now() NOT NULL,
    updated_at                  TIMESTAMPTZ                                                                                     DEFAULT now() NOT NULL,
    phone                       TEXT                                                                                   NULL     DEFAULT NULL::VARCHAR,
    phone_confirmed_at          TIMESTAMPTZ                                                                            NULL,
    phone_change                TEXT                                                                                   NULL     DEFAULT ''::VARCHAR,
    phone_change_token          VARCHAR(255)                                                                           NULL     DEFAULT ''::VARCHAR,
    phone_change_sent_at        TIMESTAMPTZ                                                                            NULL,
    confirmed_at                TIMESTAMPTZ GENERATED ALWAYS AS (LEAST(email_confirmed_at, phone_confirmed_at)) STORED NULL,
    email_change_token_current  VARCHAR(255)                                                                           NULL     DEFAULT ''::VARCHAR,
    email_change_confirm_status SMALLINT                                                                               NULL     DEFAULT 0,
    banned_until                TIMESTAMPTZ                                                                            NULL,
    reauthentication_token      VARCHAR(255)                                                                           NULL     DEFAULT ''::VARCHAR,
    reauthentication_sent_at    TIMESTAMPTZ                                                                            NULL,
    is_sso_user                 BOOLEAN                                                                                NOT NULL DEFAULT FALSE,
    deleted_at                  TIMESTAMPTZ                                                                            NULL,
    is_anonymous                BOOLEAN                                                                                NOT NULL DEFAULT FALSE,
    CONSTRAINT users_phone_key UNIQUE (phone),
    CONSTRAINT users_email_change_confirm_status_check CHECK (
        email_change_confirm_status >= 0 AND email_change_confirm_status <= 2
        )
) TABLESPACE pg_default;

-- Create indexes on the users table
CREATE INDEX IF NOT EXISTS users_instance_id_email_idx ON users USING btree (instance_id, LOWER(email::TEXT)) TABLESPACE pg_default;
CREATE UNIQUE INDEX IF NOT EXISTS confirmation_token_idx ON users USING btree (confirmation_token) TABLESPACE pg_default WHERE confirmation_token::TEXT !~ '^[0-9 ]*$';
CREATE UNIQUE INDEX IF NOT EXISTS recovery_token_idx ON users USING btree (recovery_token) TABLESPACE pg_default WHERE recovery_token::TEXT !~ '^[0-9 ]*$';
CREATE UNIQUE INDEX IF NOT EXISTS email_change_token_current_idx ON users USING btree (email_change_token_current) TABLESPACE pg_default WHERE email_change_token_current::TEXT !~ '^[0-9 ]*$';
CREATE UNIQUE INDEX IF NOT EXISTS email_change_token_new_idx ON users USING btree (email_change_token_new) TABLESPACE pg_default WHERE email_change_token_new::TEXT !~ '^[0-9 ]*$';
CREATE UNIQUE INDEX IF NOT EXISTS reauthentication_token_idx ON users USING btree (reauthentication_token) TABLESPACE pg_default WHERE reauthentication_token::TEXT !~ '^[0-9 ]*$';
CREATE UNIQUE INDEX IF NOT EXISTS users_email_partial_key ON users USING btree (email) TABLESPACE pg_default WHERE is_sso_user = FALSE;
CREATE INDEX IF NOT EXISTS users_instance_id_idx ON users USING btree (instance_id) TABLESPACE pg_default;
CREATE INDEX IF NOT EXISTS users_is_anonymous_idx ON users USING btree (is_anonymous) TABLESPACE pg_default;

-- Create an enum type for Authentication Assurance Level (AAL)
CREATE TYPE aal_level AS ENUM ('aal1', 'aal2', 'aal3');

-- AAL stands for "Authentication Assurance Level":
-- - AAL1: Low Assurance
--   - Typically during signup or initial login with unverified identity
--   - Example: User signs up with email or phone, but has not confirmed the email/phone yet.
-- - AAL2: Medium Assurance
--   - Identity partially verified, but MFA is not yet enabled
--   - Example: User confirms their email or phone during signup or login but has not set up MFA.
-- - AAL3: High Assurance
--   - Identity fully verified, and strong MFA is in place
--   - Example: User has confirmed their email/phone and has set up MFA, such as a hardware token or biometric authentication.

CREATE TABLE IF NOT EXISTS sessions
(
    id           UUID PRIMARY KEY,
    user_id      UUID                      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    created_at   TIMESTAMPTZ DEFAULT now() NOT NULL,
    updated_at   TIMESTAMPTZ DEFAULT now() NOT NULL,
    factor_id    UUID,
    aal          aal_level,
    not_after    TIMESTAMPTZ,
    refreshed_at TIMESTAMPTZ,
    user_agent   TEXT,
    ip           INET,
    tag          TEXT
);

-- Add indexes to the sessions table
CREATE INDEX IF NOT EXISTS sessions_not_after_idx ON sessions (not_after DESC);
CREATE INDEX IF NOT EXISTS sessions_user_id_idx ON sessions (user_id);
CREATE INDEX IF NOT EXISTS user_id_created_at_idx ON sessions (user_id, created_at);

-- Create the session_data table
CREATE TABLE IF NOT EXISTS session_data
(
    session_id UUID REFERENCES sessions (id) ON DELETE CASCADE,
    key        TEXT  NOT NULL,
    value      JSONB NOT NULL,
    PRIMARY KEY (session_id, key)
);

-- Create the refresh_tokens table
CREATE TABLE IF NOT EXISTS refresh_tokens
(
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token           VARCHAR(255) UNIQUE            NOT NULL,
    user_id         UUID                           NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    parent_token_id UUID,
    revoked         BOOLEAN          DEFAULT false NOT NULL,
    created_at      TIMESTAMPTZ      DEFAULT now() NOT NULL,
    updated_at      TIMESTAMPTZ      DEFAULT now() NOT NULL,
    session_id      UUID REFERENCES sessions (id) ON DELETE CASCADE,
    instance_id     UUID             DEFAULT '00000000-0000-0000-0000-000000000000'
);

-- Add indexes to the refresh_tokens table
CREATE INDEX IF NOT EXISTS refresh_tokens_instance_id_idx ON refresh_tokens (instance_id);
CREATE INDEX IF NOT EXISTS refresh_tokens_instance_id_user_id_idx ON refresh_tokens (instance_id, user_id);
CREATE INDEX IF NOT EXISTS refresh_tokens_parent_token_id_idx ON refresh_tokens (parent_token_id);
CREATE INDEX IF NOT EXISTS refresh_tokens_session_id_revoked_idx ON refresh_tokens (session_id, revoked);
CREATE INDEX IF NOT EXISTS refresh_tokens_updated_at_idx ON refresh_tokens (updated_at DESC);

-- Create the one_time_tokens table
CREATE TABLE IF NOT EXISTS one_time_tokens
(
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id    UUID REFERENCES users (id) ON DELETE CASCADE,
    token_type VARCHAR(50)                    NOT NULL,
    token      TEXT                           NOT NULL,
    metadata   JSONB,
    created_at TIMESTAMPTZ      DEFAULT now() NOT NULL,
    expires_at TIMESTAMPTZ                    NOT NULL,
    used       BOOLEAN          DEFAULT false,
    revoked    BOOLEAN          DEFAULT false
);

-- Create the password_history table
CREATE TABLE IF NOT EXISTS password_history
(
    id                 SERIAL PRIMARY KEY,
    user_id            UUID REFERENCES users (id) ON DELETE CASCADE,
    encrypted_password VARCHAR(255)              NOT NULL,
    created_at         TIMESTAMPTZ DEFAULT now() NOT NULL
);

-- Create the identities table
CREATE TABLE identities
(
    id            UUID PRIMARY KEY,
    user_id       UUID REFERENCES users (id) ON DELETE CASCADE,
    identity_type TEXT        NOT NULL, -- "email", "phone", "oauth"
    value         TEXT        NOT NULL, -- The email, phone number, or OAuth identifier
    data          JSONB,                -- Additional metadata, like OAuth claims
    verified      BOOLEAN     NOT NULL DEFAULT FALSE,
    provider      TEXT,                 -- OAuth provider (e.g., "google", "facebook")
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (identity_type, value)       -- Ensure no duplicate identities
);

-- Add indexes to the identities table
CREATE TABLE IF NOT EXISTS fido_credentials
(
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id       UUID                           NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    credential_id BYTEA                          NOT NULL, -- Unique identifier for the credential
    public_key    BYTEA                          NOT NULL, -- Public key for verifying signatures
    sign_count    BIGINT                         NOT NULL, -- Monotonically increasing counter for preventing replay attacks
    aaguid        UUID,                                    -- Authenticator Attestation GUID
    transports    TEXT[],                                  -- Transport types supported (e.g., ["usb", "nfc", "ble"])
    created_at    TIMESTAMPTZ      DEFAULT now() NOT NULL,
    last_used_at  TIMESTAMPTZ,
    display_name  VARCHAR(255),                            -- User-friendly name for the credential
    UNIQUE (user_id, credential_id)                        -- Ensures no duplicate credentials per user
);
