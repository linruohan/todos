use std::{any::type_name, collections::HashMap};

use uuid::Uuid;

use crate::enums::ObjectType;

use super::{FilterItem, Source};
pub struct BaseObject {
    pub id: String,
    pub name: String,
    pub keywords: String,
    pub icon_name: String,
    pub update_timeout_id: i32,
    pub view_id: String,
    pub filters: HashMap<String, FilterItem>,
}
impl BaseObject {
    pub fn new(name: String, keywords: String, icon_name: String, view_id: String) -> BaseObject {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            keywords,
            icon_name,
            view_id,
            filters: HashMap::new(),
            update_timeout_id: 0,
        }
    }

    pub fn get_filter(&self, id: String) -> FilterItem {
        if let Some(filter) = self.filters.get(&id) {
            filter.clone()
        } else {
            FilterItem::default()
        }
    }
    pub fn add_filter(&mut self, filter: FilterItem) {
        self.filters.insert(filter.id().clone(), filter);
    }
    pub fn update_filter(&mut self, update_filter: FilterItem) {
        if let Some(filter) = self.filters.get_mut(&update_filter.id().clone()) {
            *filter = update_filter;
        }
    }
    fn remove_filter(&mut self, filter: FilterItem) {
        if self.filters.contains_key(&filter.id().clone()) {
            self.filters.remove(&filter.id().clone());
        }
    }
    fn id_string(&self) -> String {
        self.id.clone()
    }

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
}
