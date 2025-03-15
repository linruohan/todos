use crate::{Attachment, Database, Item, Label, Project, Reminder, Section, Source};
use crate::{BaseTrait, Util};
pub struct Store {}
use once_cell::sync::OnceCell;
use std::sync::Arc;
static STOREINSTANCE: OnceCell<Arc<Store>> = OnceCell::new();
impl Store {
    pub fn new() -> Store {
        Self {}
    }
    pub fn instance() -> Arc<Store> {
        STOREINSTANCE.get_or_init(|| Arc::new(Store::new())).clone()
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

    pub fn delete_attachment(&self, attachment: Attachment) {
        if Database::default().delete_attachment(attachment.clone()) {
            self.attachments().retain(|x| *x != attachment);
        }
    }
    pub fn attachments(&self) -> Vec<Attachment> {
        Database::default().get_attachments_collection()
    }
    pub fn get_attachments_by_item(&self, item: Item) -> Vec<Attachment> {
        let mut results = Vec::new();
        for i in self.attachments() {
            if i.item_id == item.id {
                results.push(i);
            }
        }
        return results;
    }
    pub fn sources(&self) -> Vec<Source> {
        Database::default().get_sources_collection()
    }
    pub fn sections(&self) -> Vec<Section> {
        Database::default().get_sections_collection()
    }
    pub fn projects(&self) -> Vec<Project> {
        Database::default().get_projects_collection()
    }

    pub fn items(&self) -> Vec<Item> {
        Database::default().get_items_collection()
    }
    pub fn labels(&self) -> Vec<Label> {
        Database::default().get_labels_collection()
    }
    pub fn reminders(&self) -> Vec<Reminder> {
        Database::default().get_reminders_collection()
    }
    pub fn get_reminder(&self, id: String) -> Option<Reminder> {
        for reminder in self.reminders() {
            if reminder.id.clone().unwrap() == id {
                return Some(reminder);
            }
        }
        return None;
    }
    pub fn get_reminders_by_item(&self, item: Item) -> Vec<Reminder> {
        let mut results = Vec::new();
        for i in self.reminders() {
            if i.item_id == item.id {
                results.push(i);
            }
        }
        return results;
    }

    pub fn is_database_empty(&self) -> bool {
        self.projects().len() <= 0
    }
    pub fn is_sources_empty(&self) -> bool {
        self.sources().len() <= 0
    }
    pub fn get_collection_by_type(&self, obj_type: &dyn BaseTrait) -> Vec<Box<dyn BaseTrait>> {
        match obj_type.object_type() {
            crate::enums::ObjectType::PROJECT => self.projects(),
            crate::enums::ObjectType::SECTION => self.sections(),
            crate::enums::ObjectType::ITEM => self.items(),
            crate::enums::ObjectType::LABEL => self.labels(),
            _ => Vec::new(),
        }
    }
    pub fn get_project(&self, id: String) -> Option<Project> {
        for project in self.projects() {
            if project.id.clone().unwrap() == id {
                return Some(project);
            }
        }
        return None;
    }
    pub fn get_subprojects(&self, project: Project) -> Vec<Project> {
        let mut subprojects = Vec::new();
        for pro in self.projects() {
            if pro.parent_id == project.id {
                subprojects.push(pro);
            }
        }
        return subprojects;
    }
    pub fn get_item(&self, id: String) -> Option<Item> {
        for item in self.items() {
            if item.id.clone().unwrap() == id {
                return Some(item);
            }
        }
        return None;
    }
    pub fn update_item(&self, item: Item, update_id: String) {
        if (Services.Database.get_default().update_item(item, update_id)) {
            item.updated(update_id);
        }
    }
    pub fn get_subitems(&self, item: Item) -> Vec<Item> {
        let mut subitems = Vec::new();
        for it in self.items() {
            if it.parent_id == item.id {
                subitems.push(it);
            }
        }
        return subitems;
    }
    pub fn get_subitems_uncomplete(&self, item: Item) -> Vec<Item> {
        let mut subitems = Vec::new();
        for it in self.items() {
            if it.parent_id == item.id && it.checked != Some(1) {
                subitems.push(it);
            }
        }
        return subitems;
    }
    pub fn get_section(&self, id: String) -> Option<Section> {
        for section in self.sections() {
            if section.id.clone().unwrap() == id {
                return Some(section);
            }
        }
        return None;
    }
    pub fn get_items_completed(&self) -> Vec<Item> {
        let return_value = Vec::new();
        for item in self.items() {
            if item.checked == Some(1) && !item.was_archived != 1 {
                return_value.push(item);
            }
        }
        return return_value;
    }
    pub fn get_items_has_labels(&self) -> Vec<Item> {
        let return_value = Vec::new();
        for item in self.items() {
            if (item.has_labels() && item.completed == Some(0) && !item.was_archived()) {
                return_value.push(item);
            }
        }
        return return_value;
    }

    pub fn get_items_pinned(&self, checked: bool) -> Vec<Item> {
        let checked = checked as i32;
        let return_value = Vec::new();
        for item in self.items() {
            if (item.pinned == Some(1) && item.checked == Some(checked) && !item.was_archived()) {
                return_value.push(item);
            }
        }
        return return_value;
    }
    pub fn get_items_by_priority(&self, priority: i32, checked: bool) -> Vec<Item> {
        let checked = checked as i32;
        let return_value = Vec::new();
        for item in self.items() {
            if item.priority == Some(priority)
                && item.checked == Some(checked)
                && !item.was_archived()
            {
                return_value.push(item);
            }
        }

        return return_value;
    }
    pub fn get_items_with_reminders(&self) -> Vec<Item> {
        let return_value = Vec::new();
        for item in self.items() {
            if (item.has_reminders() && item.completed == Some(0) && !item.was_archived()) {
                return_value.push(item);
            }
        }
        return return_value;
    }
    pub fn get_items_by_scheduled(&self, checked: bool) -> Vec<Item> {
        let return_value = Vec::new();
        for item in self.items() {
            if (item.has_due
                && !item.was_archived()
                && item.checked == Some(checked)
                && item.due().datetime() > Local::now().naive_local())
            {
                return_value.push(item);
            }
        }
        return return_value;
    }
    pub fn get_items_by_overdeue_view(&self, checked: bool) -> Vec<Item> {
        let return_value = Vec::new();
        for item in self.items() {
            if (item.has_due == Some(1)
                && !item.was_archived()
                && item.checked == Some(checked)
                && item.due.datetime.compare(date_now) < 0
                && !Util::Datetime::is_same_day(item.due.datetime, date_now))
            {
                return_value.push(item);
            }
        }
        return return_value;
    }
    pub fn get_source(&self, id: String) -> Option<Source> {
        for source in self.sources() {
            if source.id.clone().unwrap() == id {
                return Some(source);
            }
        }
        return None;
    }
}
