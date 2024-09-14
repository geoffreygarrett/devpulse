CREATE TABLE authorization_models
(
    model_id VARCHAR(255) NOT NULL,
    config   JSONB        NOT NULL,
    PRIMARY KEY (model_id)
);

CREATE TABLE authorization_model_changelog
(
    commit_timestamp TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    model_id         VARCHAR(255) NOT NULL,
    config           JSONB        NOT NULL,
    PRIMARY KEY (commit_timestamp, model_id)
);

-- Index to speed up lookups by model_id
CREATE INDEX idx_authorization_model_changelog_model_id ON authorization_model_changelog (model_id);
