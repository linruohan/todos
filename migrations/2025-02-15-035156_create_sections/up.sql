-- Your SQL goes here
CREATE TABLE IF NOT EXISTS sections (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    archived_at TEXT NOT NULL,
    added_at TEXT NOT NULL,
    project_id TEXT NOT NULL,
    section_order INTEGER,
    collapsed INTEGER,
    is_deleted INTEGER,
    is_archived INTEGER,
    color TEXT NOT NULL,
    description TEXT NOT NULL,
    hidded INTEGER,
    FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE
);