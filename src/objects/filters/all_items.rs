use std::collections::HashMap;

use super::FilterItem;
use uuid::Uuid;

use crate::{BaseObject, objects::BaseTrait};
pub struct AllItems {
    pub base: BaseObject,
}

impl AllItems {
    pub fn get_default() -> AllItems {
        Self {
            base: BaseObject::new(
                "All Tasks".to_string(),
                format!("{};{}", "all tasks", "all"),
                "check-round-outline-symbolic".to_string(),
                "all-items-view".to_string(),
            ),
        }
    }
}
