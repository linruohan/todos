use std::{any::type_name, collections::HashMap};

use crate::enums::ObjectType;

use super::{FilterItem, Source};
pub trait BaseObject {
    fn filters(&self) -> HashMap<String, FilterItem>;
    fn add_filter(&mut self, filter: FilterItem) {
        self.filters().insert(filter.id().clone(), filter);
    }
    fn remove_filter(&mut self, filter: FilterItem) {
        if self.filters().contains_key(&filter.id().clone()) {
            self.filters().remove(&filter.id().clone());
        }
    }
    // signal
    fn deleted(&self);
    fn updated(&self, update_id: String);
    fn archived(&self);
    fn unarchived(&self);

    fn id_string(&self) -> String;
    fn loading(&self, value: bool) -> bool;
    fn sensitive(&self, value: bool) -> bool;
    fn loading_change(&self);
    fn sensitive_change(&self);

    fn view_id(&self);
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
    fn object_type(&self) -> ObjectType;
    fn object_type_string(&self) -> String;

    fn table_name(&self) -> String {
        format!("{}s", self.type_name())
    }
    fn column_order_name(&self) -> String;
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
}
