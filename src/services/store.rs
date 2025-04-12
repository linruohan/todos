use crate::schema::projects::is_archived;
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

    pub fn is_database_empty(&self) -> bool {
        self.projects().len() <= 0
    }
    pub fn is_sources_empty(&self) -> bool {
        self.sources().len() <= 0
    }
    pub fn get_collection_by_type(&self, obj_type: &dyn BaseTrait) -> Vec<Box<dyn BaseTrait>> {
        match obj_type.object_type() {
            crate::enums::ObjectType::SECTION => self.sections(),
            crate::enums::ObjectType::ITEM => self.items(),
            crate::enums::ObjectType::LABEL => self.labels(),
            crate::enums::ObjectType::PROJECT => self.projects(),
            _ => Vec::new(),
        }
    }
    // attachments
    pub fn attachments(&self) -> Vec<Attachment> {
        Database::default().get_attachments_collection()
    }
    pub fn delete_attachment(&self, attachment: Attachment) {
        if Database::default().delete_attachment(attachment.clone()) {
            self.attachments().retain(|x| *x != attachment);
        }
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

    // sources
    pub fn sources(&self) -> Vec<Source> {
        Database::default().get_sources_collection()
    }

    pub fn get_source(&self, id: String) -> Option<Source> {
        self.sources()
            .iter()
            .find(|s| s.id.as_deref() == Some(&id))
            .cloned()
    }

    pub fn insert_source(&self, source: Source) {
        let mut new_source = source;
        new_source.child_order = Some(self.sources().len() as i32 + 1);
        if Database::default().insert_source(new_source.clone()) {
            // self.sources().push(new_source.clone());
        }
    }
    pub fn delete_source(&self, source: Source) {
        if Database::default().delete_source(source.clone()) {
            for project in self.get_projects_by_source(source.id_string()) {
                self.delete_project(project);
            }
        }
    }
    pub fn update_source(&self, source: Source) {
        if Database::default().update_source(source.clone()) {
            for project in self.get_projects_by_source(source.id_string()) {
                self.delete_project(project);
            }
        }
    }
    // projects
    pub fn projects(&self) -> Vec<Project> {
        Database::default().get_projects_collection()
    }
    pub fn insert_project(&self, project: Project) {
        if Database::default().insert_project(project.clone()) {
            if let Some(parent) = project.parent() {
                parent.add_subproject(project.clone());
            }
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
    pub fn get_projects_by_source(&self, id: String) -> Vec<Project> {
        let mut results = Vec::new();
        for project in self.projects() {
            if project.source_id == Some(id.clone()) {
                results.push(project.clone());
            }
        }
        return results;
    }
    pub fn update_project(&self, project: Project) {
        if Database::default().update_project(project.clone()) {
            project.updated();
        }
    }
    pub fn delete_project(&self, project: Project) {
        if Database::default().delete_project(project) {
            for section in self.get_sections_by_project(project.id_string()) {
                self.delete_section(section);
            }
            for item in self.get_items_by_project(project.id_string()) {
                self.delete_item(item);
            }
            for project in self.get_subprojects(project.clone()) {
                self.delete_project(project);
            }
        }
    }

    pub fn archive_project(&self, project: Project) {
        if Database::default().archive_project(project.clone()) {
            for item in self.get_items_by_project(project.id_string()) {
                self.archive_item(item, project.is_archived());
            }
            for section in self.get_sections_by_project(project.id_string()) {
                section.is_archived = Some(project.is_archived() as i32);
                self.archive_section(section);
            }
        }
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
    pub fn get_inbox_project(&self) -> Vec<Project> {
        let mut results = Vec::new();
        for pro in self.projects() {
            if pro.is_inbox_project() {
                results.push(pro);
            }
        }
        return results;
    }
    // sections
    pub fn sections(&self) -> Vec<Section> {
        Database::default().get_sections_collection()
    }
    pub fn get_section(&self, id: String) -> Option<Section> {
        for section in self.sections() {
            if section.id.clone().unwrap() == id {
                return Some(section);
            }
        }
        return None;
    }
    pub fn get_sections_by_project(&self, id: String) -> Vec<Section> {
        let mut results = Vec::new();
        for section in self.sections() {
            if section.project_id == Some(id.clone()) {
                results.push(section.clone());
            }
        }
        return results;
    }
    // items
    pub fn items(&self) -> Vec<Item> {
        Database::default().get_items_collection()
    }

    pub fn insert_item(&self, item: Item, insert: bool) {
        if Database::default().insert_item(item.clone()) {
            self.add_item(item.clone(), insert);
        }
    }

    pub fn add_item(&self, item: Item, insert: bool) {
        self.items().push(item);
        // item_added (item, insert);

        if (insert) {
            if (!item.parent_id.is_none()) {
                item.parent().item_added(item);
            } else {
                if (item.section_id.is_none()) {
                    item.project().item_added(item);
                } else {
                    item.section().item_added(item);
                }
            }
        }
        // Services.EventBus.get_default ().update_items_position (item.project_id, item.section_id);
    }
    pub fn update_item(&self, item: Item, update_id: String) {
        if Database::default().update_item(item.clone()) {
            self.item_updated(item.clone(), update_id.clone());
        }
    }
    pub fn delete_item(&self, item: Item) {
        if Database::default().delete_item(item.clone()) {
            for subitem in self.get_subitems(item.clone()) {
                self.delete_item(subitem);
            }
            item.project().item_deleted(item.clone());
            if item.has_section() {
                item.section().item_deleted(item.clone());
            }
        }
    }
    pub fn archive_item(&self, item: Item, archived: bool) {
        if archived {
            item.archived();
        } else {
            item.unarchived();
        }
        for subitem in self.get_subitems(item.clone()) {
            self.archive_item(subitem, archived);
        }
    }
    pub fn get_item(&self, id: String) -> Option<Item> {
        for item in self.items() {
            if item.id.clone().unwrap() == id {
                return Some(item);
            }
        }
        return None;
    }
    pub fn get_items_by_project(&self, id: String) -> Vec<Item> {
        let mut results = Vec::new();
        for item in self.items() {
            if item.project_id == Some(id.clone()) {
                results.push(item.clone());
            }
        }
        return results;
    }
    pub fn get_items_by_section(&self, id: String) -> Vec<Item> {
        let mut results = Vec::new();
        for item in self.items() {
            if item.section_id == Some(id.clone()) {
                results.push(item.clone());
            }
        }
        return results;
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

    pub fn get_items_by_label(&self, label: Label, checked: bool) -> Vec<Item> {
        let return_value = Vec::new();
        for item in self.items() {
            if (item.has_label(label.id) && item.checked == Some(0) && !item.was_archived()) {
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
    // labels
    pub fn labels(&self) -> Vec<Label> {
        Database::default().get_labels_collection()
    }
    pub fn delete_label(&self, label: Label) {
        if Database::default().delete_label(label.clone()) {
            self.labels().retain(|x| *x != label);
        }
    }

    // reminders
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
}
