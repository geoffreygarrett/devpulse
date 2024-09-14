CREATE TABLE changelog
(
    changelog_shard_id INT          NOT NULL,
    timestamp          TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    unique_update_id   UUID         NOT NULL,
    PRIMARY KEY (changelog_shard_id, timestamp, unique_update_id),
    operation_type     VARCHAR(50)  NOT NULL, -- e.g., 'INSERT', 'DELETE'
    shard_id           INT          NOT NULL,
    object_id          VARCHAR(255) NOT NULL,
    relation           VARCHAR(255) NOT NULL,
    user_id            VARCHAR(255) NOT NULL
);

-- Index to speed up lookups by timestamp
CREATE INDEX idx_changelog_timestamp ON changelog (timestamp);
