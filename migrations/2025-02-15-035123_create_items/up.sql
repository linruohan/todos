-- Your SQL goes here
CREATE TABLE IF NOT EXISTS items (
    id TEXT PRIMARY KEY NOT NULL,
    content TEXT NOT NULL,
    description TEXT NOT NULL,
    due TEXT NOT NULL,
    added_at TEXT NOT NULL,
    completed_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    section_id TEXT NOT NULL,
    project_id TEXT NOT NULL,
    parent_id TEXT NOT NULL,
    priority INTEGER NOT NULL,
    child_order INTEGER NOT NULL,
    checked INTEGER NOT NULL,
    is_deleted INTEGER NOT NULL,
    day_order INTEGER NOT NULL,
    collapsed INTEGER NOT NULL,
    pinned INTEGER NOT NULL,
    labels TEXT NOT NULL,
    extra_data TEXT NOT NULL,
    item_type TEXT NOT NULL
);

PRAGMA foreign_keys = ON;

CREATE TRIGGER IF NOT EXISTS after_insert_item
AFTER
INSERT
    ON items BEGIN
INSERT
    OR IGNORE INTO o_events (
        event_type,
        object_id,
        object_type,
        object_key,
        object_old_value,
        object_new_value,
        parent_project_id
    )
VALUES
    (
        "insert",
        NEW.id,
        "item",
        "content",
        NEW.content,
        NEW.content,
        NEW.project_id
    );

END;

CREATE TRIGGER IF NOT EXISTS after_update_content_item
AFTER
UPDATE
    ON items FOR EACH ROW
    WHEN NEW.content != OLD.content BEGIN
INSERT
    OR IGNORE INTO o_events (
        event_type,
        object_id,
        object_type,
        object_key,
        object_old_value,
        object_new_value,
        parent_project_id
    )
VALUES
    (
        "update",
        NEW.id,
        "item",
        "content",
        OLD.content,
        NEW.content,
        NEW.project_id
    );

END;

CREATE TRIGGER IF NOT EXISTS after_update_description_item
AFTER
UPDATE
    ON items FOR EACH ROW
    WHEN NEW.description != OLD.description BEGIN
INSERT
    OR IGNORE INTO o_events (
        event_type,
        object_id,
        object_type,
        object_key,
        object_old_value,
        object_new_value,
        parent_project_id
    )
VALUES
    (
        "update",
        NEW.id,
        "item",
        "description",
        OLD.description,
        NEW.description,
        NEW.project_id
    );

END;

CREATE TRIGGER IF NOT EXISTS after_update_due_item
AFTER
UPDATE
    ON items FOR EACH ROW
    WHEN NEW.due != OLD.due BEGIN
INSERT
    OR IGNORE INTO o_events (
        event_type,
        object_id,
        object_type,
        object_key,
        object_old_value,
        object_new_value,
        parent_project_id
    )
VALUES
    (
        "update",
        NEW.id,
        "item",
        "due",
        OLD.due,
        NEW.due,
        NEW.project_id
    );

END;

CREATE TRIGGER IF NOT EXISTS after_update_priority_item
AFTER
UPDATE
    ON items FOR EACH ROW
    WHEN NEW.priority != OLD.priority BEGIN
INSERT
    OR IGNORE INTO o_events (
        event_type,
        object_id,
        object_type,
        object_key,
        object_old_value,
        object_new_value,
        parent_project_id
    )
VALUES
    (
        "update",
        NEW.id,
        "item",
        "priority",
        OLD.priority,
        NEW.priority,
        NEW.project_id
    );

END;

CREATE TRIGGER IF NOT EXISTS after_update_labels_item
AFTER
UPDATE
    ON items FOR EACH ROW
    WHEN NEW.labels != OLD.labels BEGIN
INSERT
    OR IGNORE INTO o_events (
        event_type,
        object_id,
        object_type,
        object_key,
        object_old_value,
        object_new_value,
        parent_project_id
    )
VALUES
    (
        "update",
        NEW.id,
        "item",
        "labels",
        OLD.labels,
        NEW.labels,
        NEW.project_id
    );

END;

CREATE TRIGGER IF NOT EXISTS after_update_pinned_item
AFTER
UPDATE
    ON items FOR EACH ROW
    WHEN NEW.pinned != OLD.pinned BEGIN
INSERT
    OR IGNORE INTO o_events (
        event_type,
        object_id,
        object_type,
        object_key,
        object_old_value,
        object_new_value,
        parent_project_id
    )
VALUES
    (
        "update",
        NEW.id,
        "item",
        "pinned",
        OLD.pinned,
        NEW.pinned,
        NEW.project_id
    );

END;

CREATE TRIGGER IF NOT EXISTS after_update_checked_item
AFTER
UPDATE
    ON items FOR EACH ROW
    WHEN NEW.checked != OLD.checked BEGIN
INSERT
    OR IGNORE INTO o_events (
        event_type,
        object_id,
        object_type,
        object_key,
        object_old_value,
        object_new_value,
        parent_project_id
    )
VALUES
    (
        "update",
        NEW.id,
        "item",
        "checked",
        OLD.checked,
        NEW.checked,
        NEW.project_id
    );

END;

CREATE TRIGGER IF NOT EXISTS after_update_section_item
AFTER
UPDATE
    ON items FOR EACH ROW
    WHEN NEW.section_id != OLD.section_id BEGIN
INSERT
    OR IGNORE INTO o_events (
        event_type,
        object_id,
        object_type,
        object_key,
        object_old_value,
        object_new_value,
        parent_project_id
    )
VALUES
    (
        "update",
        NEW.id,
        "item",
        "section",
        OLD.section_id,
        NEW.section_id,
        NEW.project_id
    );

END;

CREATE TRIGGER IF NOT EXISTS after_update_project_item
AFTER
UPDATE
    ON items FOR EACH ROW
    WHEN NEW.project_id != OLD.project_id BEGIN
INSERT
    OR IGNORE INTO o_events (
        event_type,
        object_id,
        object_type,
        object_key,
        object_old_value,
        object_new_value,
        parent_project_id
    )
VALUES
    (
        "update",
        NEW.id,
        "item",
        "project",
        OLD.project_id,
        NEW.project_id,
        NEW.project_id
    );

END;