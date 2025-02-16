-- Your SQL goes here
CREATE TABLE IF NOT EXISTS labels (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    color TEXT NOT NULL,
    item_order INTEGER,
    is_deleted INTEGER,
    is_favorite INTEGER,
    backend_type TEXT NOT NULL,
    source_id TEXT NOT NULL,
    CONSTRAINT unique_label UNIQUE (name)
);