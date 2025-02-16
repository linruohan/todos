-- Your SQL goes here
CREATE TABLE IF NOT EXISTS reminders (
    id INTEGER NOT NULL PRIMARY KEY,
    notify_uid INTEGER,
    item_id TEXT NOT NULL,
    service TEXT NOT NULL,
    type TEXT NOT NULL,
    due TEXT NOT NULL,
    mm_offset INTEGER,
    is_deleted INTEGER,
    FOREIGN KEY (item_id) REFERENCES items (id) ON DELETE CASCADE
);