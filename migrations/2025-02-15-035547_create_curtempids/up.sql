-- Your SQL goes here
CREATE TABLE IF NOT EXISTS cur_temp_ids (
    id TEXT NOT NULL PRIMARY KEY,
    temp_id TEXT NOT NULL,
    object TEXT NOT NULL
);