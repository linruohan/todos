use crate::BaseTrait;
use crate::{Attachment, Database, Item, Label, Project, Reminder, Section, Source};
pub struct Store {}
use crate::objects::BaseObject;
use crate::utils::DateTime;
use chrono::{Datelike, Local, NaiveDateTime};
use once_cell::sync::OnceCell;
use std::sync::Arc;

static STOREINSTANCE: OnceCell<Arc<Store>> = OnceCell::new();

pub enum Collection {
    Sections(Vec<Section>),
    Items(Vec<Item>),
    Labels(Vec<Label>),
    Projects(Vec<Project>),
    None,
}

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
    pub fn get_collection_by_type(&self, obj_type: &dyn BaseTrait) -> Collection {
        match obj_type.object_type() {
            crate::enums::ObjectType::SECTION => Collection::Sections(self.sections()),
            crate::enums::ObjectType::ITEM => Collection::Items(self.items()),
            crate::enums::ObjectType::LABEL => Collection::Labels(self.labels()),
            crate::enums::ObjectType::PROJECT => Collection::Projects(self.projects()),
            _ => Collection::None,
        }
    }

    // attachments
    pub fn attachments(&self) -> Vec<Attachment> {
        Database::default().get_attachments_collection()
    }
    pub fn delete_attachment(&self, attachment: Attachment) {
        if Database::default().delete_attachment(attachment.clone()) {
            // attachment.deleted ();
            // attachment_deleted (attachment);
            // _attachments.remove (attachment);
            //
            // attachment.item.attachment_deleted (attachment);
        }
    }

    pub fn insert_attachment(&self, attachment: Attachment) {
        if Database::default().insert_attachment(attachment.clone()) {
            // attachments.add (attachment);
            // attachment.item.attachment_added (attachment);
            todo!()
        }
    }

    pub fn get_attachments_by_item(&self, item: Item) -> Vec<Attachment> {
        self.attachments()
            .iter()
            .filter(|s| s.item_id == item.id)
            .collect()
    }

    // sources
    pub fn sources(&self) -> Vec<Source> {
        Database::default().get_sources_collection()
    }

    pub fn get_source(&self, id: &str) -> Option<Source> {
        self.sources()
            .iter()
            .find(|s| s.id.as_deref() == Some(id))
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
    pub fn get_project(&self, id: &str) -> Option<Project> {
        self.projects()
            .iter()
            .find(|s| s.id.as_deref() == Some(id))
            .cloned()
    }
    pub fn get_projects_by_source(&self, id: &str) -> Vec<Project> {
        self.projects()
            .iter()
            .find(|s| s.source_id.as_deref() == Some(id))
            .collect()
    }
    pub fn update_project(&self, project: Project) {
        if Database::default().update_project(project.clone()) {
            // project.updated();
        }
    }
    pub fn delete_project(&self, project: Project) {
        let project_id = project.id_string();
        if Database::default().delete_project(project) {
            for section in self.get_sections_by_project(&project_id) {
                self.delete_section(section);
            }
            for item in self.get_items_by_project(&project_id) {
                self.delete_item(item);
            }
            for subproject in self.get_subprojects(&project_id) {
                self.delete_project(subproject);
            }
        }
    }

    pub fn archive_project(&self, project: Project) {
        if Database::default().archive_project(project.clone()) {
            for item in self.get_items_by_project(project.id_string().as_deref()) {
                self.archive_item(item, project.is_archived());
            }
            for section in self.get_sections_by_project(project.id_string()) {
                let mut sec = section.clone();
                sec.is_archived = project.is_archived;
                self.archive_section(sec);
            }
        }
    }


    pub fn get_subprojects(&self, id: &str) -> Vec<Project> {
        self.projects()
            .iter()
            .filter(|s| s.parent_id.as_deref() == Some(id))
            .collect()
    }

    pub fn get_inbox_project(&self) -> Vec<Project> {
        self.projects()
            .iter()
            .filter(|s| s.is_inbox_project())
            .collect()
    }
    pub fn get_all_projects_archived(&self) -> Vec<Project> {
        self.projects().iter().filter(|s| s.is_archived()).collect()
    }
    pub fn get_all_projects_by_search(&self, search_text: &str) -> Vec<Section> {
        let search_lover = search_text.to_lowercase();
        self.projects()
            .iter()
            .filter(|s| s.name.contains(&search_lover) && !s.is_archived())
            .collect()
    }

    // sections
    pub fn sections(&self) -> Vec<Section> {
        Database::default().get_sections_collection()
    }
    pub fn get_section(&self, id: &str) -> Option<Section> {
        self.sections()
            .iter()
            .filter(|s| s.id.as_deref() == Some(id))
            .collect()
    }
    pub fn get_sections_by_project(&self, id: &str) -> Vec<Section> {
        self.sections()
            .iter()
            .find(|s| s.project_id.as_deref() == Some(id))
            .collect()
    }
    pub fn get_all_sections_by_search(&self, search_text: &str) -> Vec<Section> {
        let search_lover = search_text.to_lowercase();
        self.sections()
            .iter()
            .filter(|s| {
                s.name
                    .as_deref()
                    .map(|name| name.contains(&search_lover))
                    .unwrap_or(false)
                    && !s.was_archived()
            })
            .collect()
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
        let mut item1 = item.clone();
        // self.items().push(item);
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
            // self.item_updated(item.clone(), update_id.clone());
        }
    }
    pub fn update_item_pin(&self, item: Item) {
        if Database::default().update_item(item.clone()) {
            item.pin_updated();
        }
    }
    pub fn move_item(&self, item: &Item, project_id: &str, section_id: &str) {
        if Database::default().move_item(&item) {
            for subitem in self.get_subitems(item) {
                let mut sub = subitem.clone();
                sub.project_id = item.project_id.clone();
                self.move_item(&sub, "", "");
            }
            if let Some(section_id) = item.section_id.clone() {
                if let Some(section) = self.get_section(&section_id) {
                    section.update_count();
                }
            }
            if let Some(project_id) = item.project_id.clone() {
                if let Some(project) = self.get_project(&project_id) {
                    project.update_count();
                }
            }
        }
    }

    pub fn delete_item(&self, item: &Item) {
        if Database::default().delete_item(item.clone()) {
            for subitem in self.get_subitems(item) {
                self.delete_item(&subitem);
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
        for subitem in self.get_subitems(&item) {
            self.archive_item(subitem, archived);
        }
    }

    pub fn get_item(&self, id: &str) -> Option<Item> {
        self.items()
            .iter()
            .find(|i| i.id.as_deref() == Some(id))
            .collect()
    }

    pub fn get_items_by_section(&self, id: &str) -> Vec<Item> {
        self.items()
            .iter()
            .filter(|s| s.section_id.as_deref() == Some(id))
            .collect()
    }

    pub fn get_subitems(&self, item: &Item) -> Vec<Item> {
        self.items()
            .iter()
            .filter(|s| s.parent_id.as_deref() == Some(item.id_string()))
            .collect()
    }

    pub fn get_items_completed(&self) -> Vec<Item> {
        self.items()
            .iter()
            .filter(|s| s.checked == Some(1) && !s.was_archived())
            .collect()
    }
    pub fn get_items_has_labels(&self) -> Vec<Item> {
        self.items()
            .iter()
            .filter(|s| s.has_labels() && s.completed() && !s.was_archived())
            .collect()
    }

    pub fn get_items_by_label(&self, label_id: &str, checked: bool) -> Vec<Item> {
        self.items()
            .iter()
            .filter(|i| i.has_label(&label_id) && i.checked() == checked && !i.was_archived())
            .collect()
    }
    pub fn get_item_by_baseobject(&self, baseobject: BaseObject) -> Vec<Item> {}
    pub fn get_items_checked(&self) -> Vec<Item> {
        self.items().iter().filter(|i| i.checked()).collect()
    }
    pub fn get_items_checked_by_project(&self, project: &Project) -> Vec<Item> {
        self.items().iter().filter(|i| i.project_id == project.id && i.checked()).collect()
    }
    pub fn get_subitems_uncomplete(&self, item: Item) -> Vec<Item> {
        self.items().iter().filter(|i| i.parent_id == i.id && !i.checked()).collect()
    }
    pub fn get_items_by_project(&self, project: &Project) -> Vec<Item> {
        self.items().iter().filter(|i| i.exists_project(project)).collect()
    }
    pub fn get_items_by_project_pinned(&self, project: &Project) -> Vec<Item> {
        self.items().iter().filter(|i| i.exists_project(project) && i.pinned()).collect()
    }
    pub fn get_items_by_date(&self, date: &NaiveDateTime, checked: bool) -> Vec<Item> {
        self.items().iter().filter(|i| self.valid_item_by_date(i, date, checked)).collect()
    }
    pub fn get_items_no_date(&self, checked: bool) -> Vec<Item> {
        self.items().iter().filter(|i| !i.has_due() && i.checked() == checked).collect()
    }
    pub fn get_items_repeating(&self, checked: bool) -> Vec<Item> {
        self.items().iter().filter(|i| i.has_due() && i.due().is_recurring && i.checked() == checked && !i.was_archived()).collect()
    }
    pub fn get_items_by_date_range(&self, start_date: &NaiveDateTime, end_date: &NaiveDateTime, checked: bool) -> Vec<Item> {
        self.items().iter().filter(|s| self.valid_item_by_date_range(s, start_date, end_date, checked)).collect()
    }
    pub fn get_items_by_month(&self, date: &NaiveDateTime, checked: bool) -> Vec<Item> {
        self.items().iter().filter(|s| self.valid_item_by_month(s, date, checked)).cloned().collect()
    }
    pub fn get_items_pinned(&self, checked: bool) -> Vec<Item> {
        self.items()
            .iter()
            .filter(|i| i.pinned == Some(1) && i.checked() && !i.was_archived())
            .collect()
    }
    pub fn get_items_by_priority(&self, priority: i32, checked: bool) -> Vec<Item> {
        self.items()
            .iter()
            .filter(|i| i.priority == Some(priority) && i.checked() && !i.was_archived())
            .collect()
    }
    pub fn get_items_with_reminders(&self) -> Vec<Item> {
        self.items()
            .iter()
            .filter(|i| i.has_reminders() && i.completed() && !i.was_archived())
            .collect()
    }
    pub fn get_items_by_scheduled(&self, checked: bool) -> Vec<Item> {
        self.items()
            .iter()
            .filter(|i| {
                i.has_due()
                    && !i.was_archived()
                    && i.checked()
                    && i.due().datetime() > Local::now().naive_local()
            })
            .collect()
    }

    pub fn get_items_unlabeled(&self, checked: bool) -> Vec<Item> {
        self.items().iter().filter(|s| s.labels().len() <= 0 && s.checked() == checked && !s.was_archived()).collect()
    }
    pub fn get_items_no_parent(&self, checked: bool) -> Vec<Item> {
        self.items().iter().filter(|i| !i.was_archived() &&
            i.checked() == checked &&
            !i.has_parent()).cloned().collect()
    }
    pub fn valid_item_by_date(&self, item: &Item, date: &NaiveDateTime, checked: bool) -> bool {
        if item.has_due() || item.was_archived() {
            return false;
        }
        item.checked() == checked && DateTime::default().is_same_day(&item.due().datetime(), date)
    }

    pub fn valid_item_by_date_range(&self, item: &Item, start_date: &NaiveDateTime, end_date: &NaiveDateTime, checked: bool) -> bool {
        if item.has_due() || item.was_archived() {
            return false;
        }
        let date = DateTime::default().get_date_only(&item.due().datetime());
        let start = DateTime::default().get_date_only(start_date);
        let end = DateTime::default().get_date_only(end_date);
        item.checked() == checked && date >= start && date <= end
    }
    pub fn valid_item_by_month(&self, item: &Item, date: &NaiveDateTime, checked: bool) -> bool {
        if item.has_due() || item.was_archived() {
            return false;
        }

        item.checked() == checked && item.due().datetime().month() == date.month() && item.due().datetime().year() == date.year()
    }

    pub fn get_items_by_overdeue_view(&self, checked: bool) -> Vec<Item> {
        let date_now = Local::now().naive_local();
        self.items()
            .iter()
            .filter(|i| {
                i.has_due()
                    && !i.was_archived()
                    && i.checked()
                    && i.due().datetime() < date_now
                    && !DateTime::default().is_same_day(&i.due().datetime(), &date_now)
            })
            .collect()
    }

    pub fn get_all_items_by_search(&self, search_text: &str) -> Vec<Item> {
        let search_lower = search_text.to_lowercase();
        self.items()
            .iter()
            .filter(|i| {
                !i.checked()
                    && !i.was_archived()
                    && (i.content.to_lowercase().contains(&search_lower)
                        || i.description
                            .as_deref()
                            .map(|desc| desc.to_lowercase().contains(&search_lower))
                            .unwrap_or(false))
            })
            .collect()
    }

    pub fn valid_item_by_overdue(&self, item: Item, checked: bool) -> bool {
        if !item.has_due() || item.was_archived() {
            return false;
        }
        let date_now = Local::now().naive_local();
        item.due().datetime() <= date_now
            && DateTime::default().is_same_day(&item.due().datetime(), &date_now)
    }

    // labels
    pub fn labels(&self) -> Vec<Label> {
        Database::default().get_labels_collection()
    }
    pub fn insert_label(&self, label: Label) {
        if Database::default().insert_label(label) {
            todo!()
        }
    }
    pub fn update_label(&self, label: Label) {
        if Database::default().update_label(label) {
            // label.updated ();
            // label_updated (label);
            todo!()
        }
    }
    pub fn delete_label(&self, label: Label) {
        if Database::default().delete_label(label) {
            // label.deleted ();
            // label_deleted (label);
            // _labels.remove (label);
            todo!()
        }
    }
    pub fn label_exists(&self, id: &str) -> bool {
        self.labels()
            .iter()
            .find(|s| s.id.as_deref() == Some(id))
            .is_some()
    }
    pub fn get_label(&self, id: &str) -> Option<Label> {
        self.labels()
            .iter()
            .find(|s| s.id.as_deref() == Some(id))
            .cloned()
    }
    pub fn get_labels_by_item_labels(&self, labels: &str) -> Vec<Label> {
        labels
            .split(";")
            .filter(|id| self.get_label(id).is_some())
            .collect()
    }
    pub fn get_label_by_name(&self, name: &str, source_id: &str) -> Option<Label> {
        self.labels()
            .iter()
            .find(|l| {
                l.name
                    .as_deref()
                    .map_or(false, |n| n.eq_ignore_ascii_case(name))
                    && l.source_id.as_deref() == Some(source_id)
            })
            .cloned()
    }
    pub fn get_labels_by_source(&self, id: &str) -> Vec<Label> {
        self.labels()
            .iter()
            .filter(|l| l.source_id.as_deref() == Some(id))
            .collect()
    }
    pub fn get_all_labels_by_search(&self, search_text: &str) -> Vec<Section> {
        let search_lover = search_text.to_lowercase();
        self.labels()
            .iter()
            .filter(|s| {
                s.name
                    .as_deref()
                    .map(|name| name.contains(&search_lover))
                    .unwrap_or(false)
                    && !s.is_archived()
            })
            .collect()
    }
    // reminders
    pub fn reminders(&self) -> Vec<Reminder> {
        Database::default().get_reminders_collection()
    }
    pub fn get_reminder(&self, id: String) -> Option<Reminder> {
        self.reminders().iter().find(|s| s.id == Some(id)).cloned()
    }

    pub fn get_reminders_by_item(&self, item: Item) -> Vec<Reminder> {
        self.reminders()
            .iter()
            .filter(|s| s.item_id == item.id)
            .collect()
    }
    pub fn insert_reminder(&self, reminder: Reminder) {
        if Database::default().insert_reminder(reminder) {
            // reminders.add (reminder);
            // reminder_added (reminder);
            // reminder.item.reminder_added (reminder);
            todo!()
        }
    }
    pub fn delete_reminder(&self, reminder: Reminder) {
        if Database::default().delete_reminder(reminder) {
            // reminder.deleted ();
            // reminder_deleted (reminder);
            // _reminders.remove (reminder);
            //
            // reminder.item.reminder_deleted (reminder);
        }
    }
}
