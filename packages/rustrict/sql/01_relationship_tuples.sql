CREATE TABLE relation_tuples
(
    shard_id         INT          NOT NULL,
    object_id        VARCHAR(255) NOT NULL,
    relation         VARCHAR(255) NOT NULL,
    user_id          VARCHAR(255) NOT NULL,
    commit_timestamp TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (shard_id, object_id, relation, user_id, commit_timestamp)
);

-- Index to speed up lookups by object ID
CREATE INDEX idx_relation_tuples_object_id ON relation_tuples (object_id);

-- Index to speed up lookups by object ID and relation
CREATE INDEX idx_relation_tuples_object_id_relation ON relation_tuples (object_id, relation);
