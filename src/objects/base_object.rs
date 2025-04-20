use crate::enums::ObjectType;
use std::{any::type_name, collections::HashMap};

use super::{FilterItem, Item, Label, Project, Reminder, Section, Source};
#[derive(Debug, Clone)]
pub struct BaseObject {
    pub name: String,
    pub keywords: String,
    pub icon_name: String,
    pub view_id: String,
    pub update_timeout_id: u32,
    pub filters: HashMap<String, FilterItem>,
}
impl BaseObject {
    pub fn new(name: String, keywords: String, icon_name: String, view_id: String) -> BaseObject {
        Self {
            name,
            keywords,
            icon_name,
            view_id,
            update_timeout_id: 0,
            filters: HashMap::new(),
        }
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn set_name(&mut self, value: impl Into<String>) {
        self.name = value.into();
    }
    fn keywords(&self) -> &str {
        &self.keywords
    }
    fn set_keywords(&mut self, keywords: impl Into<String>) {
        self.keywords = keywords.into();
    }
    fn icon_name(&self) -> &str {
        &self.icon_name
    }

    fn set_icon_name(&mut self, icon_name: impl Into<String>) {
        self.icon_name = icon_name.into();
    }
    fn view_id(&self) -> &str {
        &self.view_id
    }

    fn set_view_id(&mut self, view_id: impl Into<String>) {
        self.view_id = view_id.into();
    }
    fn update_timeout_id(&self) -> u32 {
        self.update_timeout_id
    }

    fn set_update_timeout_id(&mut self, timeout_id: u32) {
        self.update_timeout_id = timeout_id;
    }
    pub fn loading(&self) -> bool {
        false
    }
    pub fn loading_change(&self) {}
    pub fn sensitive(&self) -> bool {
        false
    }
    pub fn sensitive_change(&self) {}
    pub fn get_filter(&self, id: String) -> FilterItem {
        if let Some(filter) = self.filters.get(&id) {
            filter.clone()
        } else {
            FilterItem::default()
        }
    }
    pub fn add_filter(&mut self, filter: FilterItem) {
        if !self.filters.contains_key(&filter.id().clone()) {
            self.filters.insert(filter.id().clone(), filter);
        }
    }
    pub fn update_filter(&mut self, update_filter: FilterItem) {
        if let Some(filter) = self.filters.get_mut(&update_filter.id().clone()) {
            *filter = update_filter;
        }
    }
    pub fn remove_filter(&mut self, filter: FilterItem) {
        if self.filters.contains_key(&filter.id().clone()) {
            self.filters.remove(&filter.id().clone());
        }
    }
}
