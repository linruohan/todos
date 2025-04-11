use crate::enums::{ItemType, SourceType};
use crate::objects::{BaseObject, BaseTrait, DueDate};
use crate::schema::items;
use crate::utils::{self, EMPTY_DATETIME};
use crate::{Attachment, Database, Label, Project, Reminder, Section, Store, Util, constants};
use chrono::{Local, NaiveDateTime};
use derive_builder::Builder;
use diesel::QueryDsl;
use diesel::Queryable;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
#[derive(
    QueryableByName, Builder, Clone, Queryable, PartialEq, Eq, Selectable, Serialize, Debug,
)]
#[diesel(table_name = items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Item {
    pub id: Option<String>,
    #[builder(default, setter(strip_option))]
    pub content: String,
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    #[builder(default=Some(DueDate::default().to_string()), setter(strip_option))]
    pub due: Option<String>,
    #[builder(default=Some(Local::now().naive_local().to_string()), setter(strip_option))]
    pub added_at: Option<String>,
    #[builder(default, setter(strip_option))]
    pub completed_at: Option<String>,
    #[builder(default, setter(strip_option))]
    pub updated_at: Option<String>,
    #[builder(default, setter(strip_option))]
    pub section_id: Option<String>,
    #[builder(default, setter(strip_option))]
    pub project_id: Option<String>,
    #[builder(default, setter(strip_option))]
    pub parent_id: Option<String>,
    #[builder(default=Some(constants::PRIORITY_1), setter(strip_option))]
    pub priority: Option<i32>,
    #[builder(default, setter(strip_option))]
    pub child_order: Option<i32>,
    #[builder(default, setter(strip_option))]
    pub checked: Option<i32>,
    #[builder(default, setter(strip_option))]
    pub is_deleted: Option<i32>,
    #[builder(default, setter(strip_option))]
    pub day_order: Option<i32>,
    #[builder(default, setter(strip_option))]
    pub collapsed: Option<i32>,
    #[builder(default, setter(strip_option))]
    pub pinned: Option<i32>,
    #[builder(default, setter(strip_option))]
    pub labels: Option<String>,
    #[builder(default, setter(strip_option))]
    pub extra_data: Option<String>,
    #[builder(default=Some(ItemType::TASK.to_string()), setter(strip_option))]
    pub item_type: Option<String>,
}
impl Item {
    pub fn due(&self) -> DueDate {
        match &self.due {
            Some(due) => serde_json::from_str::<DueDate>(&due).expect("failed to convert due date"),
            None => DueDate::default(),
        }
    }
    pub fn labels(&self) -> Vec<Label> {
        match self.labels.clone() {
            Some(labels) => {
                serde_json::from_str::<Vec<Label>>(&labels).expect("failed to convert labels")
            }
            None => Vec::<Label>::new(),
        }
    }
    pub fn get_label(&self, id: String) -> Option<Label> {
        for label in self.labels() {
            if label.id.clone().unwrap() == id {
                return Some(label);
            }
        }
        return None;
    }
    pub fn get_label_by_name(&self, name: String, labels_list: Vec<Label>) -> Option<Label> {
        for label in labels_list {
            if label.name.clone().unwrap() == name {
                return Some(label);
            }
        }
        return None;
    }
    pub fn short_content(&self) -> String {
        Util::get_default().get_short_name(self.content.clone(), 0)
    }
    pub fn priority_icon(&self) -> String {
        match self.priority {
            Some(constants::PRIORITY_1) => "priority-icon-1".to_string(),
            Some(constants::PRIORITY_2) => "priority-icon-2".to_string(),
            Some(constants::PRIORITY_3) => "priority-icon-3".to_string(),
            _ => "planner-flag".to_string(),
        }
    }

    pub fn priority_color(&self) -> String {
        match self.priority {
            Some(constants::PRIORITY_1) => "#ff7066".to_string(),
            Some(constants::PRIORITY_2) => "#ff9914".to_string(),
            Some(constants::PRIORITY_3) => "#5297ff".to_string(),
            _ => "@text_color".to_string(),
        }
    }

    pub fn priority_text(&self) -> String {
        match self.priority {
            Some(constants::PRIORITY_1) => "Priority 1: high".to_string(),
            Some(constants::PRIORITY_2) => "Priority 2: medium".to_string(),
            Some(constants::PRIORITY_3) => "Priority 3: low".to_string(),
            _ => "Priority 4: none".to_string(),
        }
    }
    pub fn pinned_icon(&self) -> String {
        match self.pinned {
            Some(1) => "planner-pin-tack".to_string(),
            _ => "planner-pinned".to_string(),
        }
    }
    pub fn completed(&self) -> bool {
        return self.checked == Some(1);
    }
    pub fn has_due(&self) -> bool {
        self.due().datetime() != EMPTY_DATETIME
    }
    pub fn has_time(&self) -> bool {
        if self.due().datetime() == EMPTY_DATETIME {
            return false;
        }
        return utils::DateTime::default().has_time(self.due().datetime());
    }
    pub fn completed_date(&self) -> NaiveDateTime {
        match self.completed_at.clone() {
            Some(completed_at) => utils::DateTime::default().get_date_from_string(completed_at),
            None => EMPTY_DATETIME,
        }
    }
    pub fn has_parent(&self) -> bool {
        Store::instance().get_item(self.parent_id.clone().unwrap()) == None
    }
    pub fn has_section(&self) -> bool {
        Store::instance().get_section(self.section_id.clone().unwrap()) == None
    }
    pub fn added_datetime(&self) -> NaiveDateTime {
        match self.added_at.clone() {
            Some(added_at) => utils::DateTime::default().get_date_from_string(added_at),
            None => EMPTY_DATETIME,
        }
    }
    pub fn updated_datetime(&self) -> NaiveDateTime {
        match self.updated_at.clone() {
            Some(updated_at) => utils::DateTime::default().get_date_from_string(updated_at),
            None => EMPTY_DATETIME,
        }
    }
    pub fn parent(&self) -> Option<Item> {
        Store::instance().get_item(self.parent_id.clone().unwrap())
    }
    pub fn project(&self) -> Option<Project> {
        Store::instance().get_project(self.project_id.clone().unwrap())
    }
    pub fn section(&self) -> Option<Section> {
        Store::instance().get_section(self.section_id.clone().unwrap())
    }
    pub fn items(&self) -> Vec<Item> {
        // subitems
        let mut items = Store::instance().get_subitems(self.clone());
        items.sort_by(|a, b| a.child_order.cmp(&b.child_order));
        return items;
    }
    pub fn items_uncomplete(&self) -> Vec<Item> {
        Store::instance().get_subitems_uncomplete(self.clone())
    }
    pub fn reminders(&self) -> Vec<Reminder> {
        Store::instance().get_reminders_by_item(self.clone())
    }
    pub fn attachments(&self) -> Vec<Attachment> {
        Store::instance().get_attachments_by_item(self.clone())
    }

    pub fn get_caldav_categories(&self) {}
    pub fn check_labels(&self, new_labels: HashMap<String, Label>) {
        for (key, label) in &new_labels {
            if self.get_label(label.id.clone().unwrap()) == None {
                self.add_label_if_not_exist(label.clone());
            }
        }
        for label in self.labels() {
            if !new_labels.contains_key(&label.id.clone().unwrap()) {
                self.delete_item_label(label.id.clone().unwrap());
            }
        }
    }
    pub fn set_section(&mut self, section: Section) {
        self.section_id = Some(section.id.clone().unwrap());
    }
    pub fn set_project(&mut self, project: Project) {
        self.project_id = Some(project.id.clone().unwrap());
    }
    pub fn set_parent(&mut self, parent: Item) {
        self.parent_id = Some(parent.id.clone().unwrap());
    }

    fn add_label_if_not_exist(&self, label: Label) {
        todo!()
    }

    pub fn delete_item_label(&self, unwrap: String) -> _ {
        todo!()
    }
    pub fn update_local(&self) {
        Store::instance().update_item(self.clone(), "".to_string());
    }
    pub fn update(&self, update_id: String) {
        if let Some(project) = self.project() {
            match project.source_type() {
                SourceType::LOCAL => {
                    Store::instance().update_item(self.clone(), update_id);
                }
                _ => {}
            }
        }
    }
}

impl BaseTrait for Item {
    fn source(&self) -> crate::Source {
        todo!()
    }

    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}
