use serde::{Deserialize, Serialize};

use crate::enums::FilterItemType;
#[derive(Clone, Debug, Serialize)]
pub struct FilterItem {
    pub filter_type: FilterItemType,
    pub name: String,
    pub value: String,
}
impl FilterItem {
    pub fn default() -> FilterItem {
        Self {
            filter_type: todo!(),
            name: todo!(),
            value: todo!(),
        }
    }
    pub fn id(&self) -> String {
        match self.filter_type {
            FilterItemType::DueDate | FilterItemType::SECTION => format!("{:?}", self.filter_type),
            _ => format!("{:?}-{}", self.filter_type, self.value),
        }
    }
}
