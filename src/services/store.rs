use crate::{Attachment, Database, Item, Label, Project, Reminder, Section, Source};

pub struct Store {}

impl Store {
    pub fn instance() -> Store {
        Store {}
    }
    // signal
    pub fn source_added(source: Source) {}
    pub fn source_deleted(source: Source) {}
    pub fn source_updated(source: Source) {}

    pub fn project_added(project: Project) {}
    pub fn project_updated(project: Project) {}
    pub fn project_deleted(project: Project) {}
    pub fn project_archived(project: Project) {}
    pub fn project_unarchived(project: Project) {}

    pub fn label_added(label: Label) {}
    pub fn label_updated(label: Label) {}
    pub fn label_deleted(label: Label) {}

    pub fn section_added(section: Section) {}
    pub fn section_moved(section: Section, old_project_id: String) {}
    pub fn section_archived(section: Section) {}
    pub fn section_unarchived(section: Section) {}

    pub fn item_added(item: Item, insert: bool) {}
    pub fn item_deleted(item: Item) {}
    pub fn item_updated(item: Item, update_id: String) {}
    pub fn item_archived(item: Item) {}
    pub fn item_unarchived(item: Item) {}
    pub fn item_pin_change(item: Item) {}

    pub fn item_label_added(label: Label) {}
    pub fn item_label_deleted(label: Label) {}

    pub fn reminder_added(reminder: Reminder) {}
    pub fn reminder_deleted(reminder: Reminder) {}

    pub fn attachment_deleted(attachment: Attachment) {}

    pub fn sources() -> Vec<Source> {
        Database::default().get_sources_collection()
    }
}
