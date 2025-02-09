use std::collections::HashMap;

use super::FilterItem;
use uuid::Uuid;

use crate::BaseObject;
pub struct AllItems {
    id: String,
    name: String,
    keywords: String,
    icon_name: String,
    view_id: String,
    filters: HashMap<String, FilterItem>, // Changed to Vec<FilterType> to fix the error.
}

impl AllItems {
    pub fn get_default(&self) -> AllItems {
        AllItems {
            id: Uuid::new_v4().to_string(),
            name: "All Tasks".to_string(),
            keywords: format!("{};{}", "all tasks", "all"),
            icon_name: "check-round-outline-symbolic".to_string(),
            view_id: "all-items-view".to_string(),
            filters: HashMap::new(), // Initialize with an empty vector or default filters.
        }
    }
}
impl BaseObject for AllItems {
    fn filters(&self) -> HashMap<String, FilterItem> {
        self.filters.clone()
    }

    fn deleted(&self) {
        todo!()
    }

    fn updated(&self, update_id: String) {
        todo!()
    }

    fn archived(&self) {
        todo!()
    }

    fn unarchived(&self) {
        todo!()
    }

    fn id_string(&self) -> String {
        todo!()
    }

    fn loading(&self, value: bool) -> bool {
        todo!()
    }

    fn sensitive(&self, value: bool) -> bool {
        todo!()
    }

    fn loading_change(&self) {
        todo!()
    }

    fn sensitive_change(&self) {
        todo!()
    }

    fn view_id(&self) {
        todo!()
    }

    fn object_type(&self) -> crate::enums::ObjectType {
        todo!()
    }

    fn object_type_string(&self) -> String {
        todo!()
    }

    fn column_order_name(&self) -> String {
        todo!()
    }

    fn source(&self) -> crate::objects::Source {
        todo!()
    }
}
