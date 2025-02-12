use std::collections::HashMap;

use super::FilterItem;
use uuid::Uuid;

use crate::{BaseObject, objects::BaseTrait};
pub struct AllItems {
    pub base: BaseObject,
}

impl AllItems {
    pub fn get_default(&self) -> AllItems {
        AllItems {
            base: BaseObject::new(
                "All Tasks".to_string(),
                format!("{};{}", "all tasks", "all"),
                "check-round-outline-symbolic".to_string(),
                "all-items-view".to_string(),
            ),
        }
    }
}
impl BaseTrait for AllItems {
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

    fn loading(&self) -> bool {
        todo!()
    }

    fn loading_change(&self) {
        todo!()
    }

    fn sensitive(&self) -> bool {
        todo!()
    }

    fn sensitive_change(&self) {
        todo!()
    }

    fn source(&self) -> crate::objects::Source {
        todo!()
    }
}
