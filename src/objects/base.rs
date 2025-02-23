use crate::enums::ObjectType;
use std::{any::type_name, collections::HashMap};

use super::{FilterItem, Item, Label, Project, Reminder, Section, Source};
pub trait BaseTrait {
    fn type_name(&self) -> &str {
        let full_name = type_name::<Self>();
        full_name.split("::").last().unwrap()
    }
    fn type_delete(&self) -> String {
        format!("{}_delete", self.type_name().to_lowercase())
    }
    fn type_add(&self) -> String {
        format!("{}_add", self.type_name().to_lowercase())
    }
    fn type_update(&self) -> String {
        format!("{}_update", self.type_name().to_lowercase())
    }
    fn object_type(&self) -> ObjectType {
        match self.type_name() {
            "Item" => ObjectType::ITEM,
            "Section" => ObjectType::SECTION,
            "Project" => ObjectType::PROJECT,
            "Label" => ObjectType::LABEL,
            _ => ObjectType::FILTER,
        }
    }
    fn object_type_string(&self) -> &str {
        match self.type_name() {
            "Item" => "item",
            "Section" => "section",
            "Project" => "project",
            "Label" => "label",
            _ => "filter",
        }
    }

    fn table_name(&self) -> String {
        format!("{}s", self.type_name())
    }
    fn column_order_name(&self) -> &str {
        match self.type_name() {
            "Item" => "child_order",
            "Section" => "section_order",
            "Project" => "child_order",
            "Label" => "item_order",
            _ => "",
        }
    }
    fn source(&self) -> Source;
    fn get_update_json(&self, uuid: String, temp_id: String) -> &str {
        ""
    }

    fn get_add_json(&self, temp_id: String, uuid: String) -> &str {
        ""
    }
    fn get_move_json(&self, new_project_id: String, uuid: String) -> &str {
        ""
    }
    fn to_json(&self) -> &str {
        ""
    }
    fn filters(&self) -> HashMap<String, FilterItem>;
    fn get_filter(&self, id: String) -> FilterItem {
        if let Some(filter) = self.filters().get(&id) {
            filter.clone()
        } else {
            FilterItem::default()
        }
    }
    fn add_filter(&mut self, filter: FilterItem) {
        self.filters().insert(filter.id().clone(), filter);
    }
    fn update_filter(&mut self, update_filter: FilterItem) {
        if let Some(filter) = self.filters().get_mut(&update_filter.id().clone()) {
            *filter = update_filter;
        }
    }
    fn remove_filter(&mut self, filter: FilterItem) {
        if self.filters().contains_key(&filter.id().clone()) {
            self.filters().remove(&filter.id().clone());
        }
    }
    fn id(&self) -> Option<&str>;
    fn id_string(&self) -> String {
        self.id().unwrap().to_string()
    }
}
