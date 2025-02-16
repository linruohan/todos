-- Your SQL goes here
CREATE TABLE IF NOT EXISTS sources (
    id TEXT NOT NULL PRIMARY KEY,
    source_type TEXT NOT NULL,
    display_name TEXT NOT NULL,
    added_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    is_visible INTEGER,
    child_order INTEGER,
    sync_server INTEGER,
    last_sync TEXT NOT NULL,
    data TEXT NOT NULL
);