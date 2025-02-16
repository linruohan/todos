-- Your SQL goes here
CREATE TABLE IF NOT EXISTS queue (
    uuid TEXT NOT NULL PRIMARY KEY,
    object_id TEXT NOT NULL,
    query TEXT NOT NULL,
    temp_id TEXT NOT NULL,
    args TEXT NOT NULL,
    date_added TEXT NOT NULL
);