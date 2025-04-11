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
    // signal
    pub fn deleted(&self) {}
    pub fn updated(&self, update_id: String) {}
    pub fn archived(&self) {}
    pub fn unarchived(&self) {}
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
