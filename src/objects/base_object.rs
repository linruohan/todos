use crate::enums::ObjectType;
use std::{any::type_name, collections::HashMap};

use super::{FilterItem, Item, Label, Project, Reminder, Section, Source};
pub struct BaseObject {
    pub name: String,
    pub keywords: String,
    pub icon_name: String,
    pub update_timeout_id: String,
    pub filters: HashMap<String, FilterItem>,
}
impl BaseObject {
    // signal
    fn deleted(&self) {}
    fn updated(&self, update_id: String) {}
    fn archived(&self) {}
    fn unarchived(&self) {}
    fn loading(&self) -> bool {
        false
    }
    fn loading_change(&self) {}
    fn sensitive(&self) -> bool {
        false
    }
    fn sensitive_change(&self) {}

    fn get_filter(&self, id: String) -> FilterItem {
        if let Some(filter) = self.filters.get(&id) {
            filter.clone()
        } else {
            FilterItem::default()
        }
    }
    fn add_filter(&mut self, filter: FilterItem) {
        self.filters.insert(filter.id().clone(), filter);
    }
    fn update_filter(&mut self, update_filter: FilterItem) {
        if let Some(filter) = self.filters.get_mut(&update_filter.id().clone()) {
            *filter = update_filter;
        }
    }
    fn remove_filter(&mut self, filter: FilterItem) {
        if self.filters.contains_key(&filter.id().clone()) {
            self.filters.remove(&filter.id().clone());
        }
    }
}
