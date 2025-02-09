use std::{any::Any, collections::HashMap};

use uuid::Uuid;

use super::FilterItem;
use crate::BaseObject;
pub struct Anytime {
    id: String,
    name: String,
    keywords: String,
    icon_name: String,
    view_id: String,
    filters: HashMap<String, FilterItem>,
}

impl Anytime {
    pub fn default() -> Anytime {
        Anytime {
            name: "Anytime".to_string(),
            keywords: format!("{};{};{}", "anytime", "filters", "no date"),
            icon_name: "grid-large-symbolic".to_string(),
            view_id: "anytime-view".to_string(),
            id: Uuid::new_v4().to_string(),
            filters: HashMap::new(),
        }
    }
}

impl BaseObject for Anytime {
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
