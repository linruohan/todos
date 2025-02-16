-- Your SQL goes here
CREATE TABLE IF NOT EXISTS o_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_type TEXT NOT NULL,
    event_date DATETIME DEFAULT (datetime('now', 'localtime')),
    object_id TEXT NOT NULL,
    object_type TEXT NOT NULL,
    object_key TEXT NOT NULL,
    object_old_value TEXT NOT NULL,
    object_new_value TEXT NOT NULL,
    parent_item_id TEXT NOT NULL,
    parent_project_id TEXT NOT NULL
);