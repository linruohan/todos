use std::{any::Any, collections::HashMap};

use uuid::Uuid;

use super::FilterItem;
use crate::{BaseObject, objects::BaseTrait};
pub struct Anytime {
    pub base: BaseObject,
}

impl Anytime {
    pub fn default() -> Anytime {
        Anytime {
            base: BaseObject::new(
                "Anytime".to_string(),
                format!("{};{};{}", "anytime", "filters", "no date"),
                "grid-large-symbolic".to_string(),
                "anytime-view".to_string(),
            ),
        }
    }
}

impl BaseTrait for Anytime {
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

    fn loading_change(&self) {
        todo!()
    }

    fn sensitive_change(&self) {
        todo!()
    }

    fn source(&self) -> crate::objects::Source {
        todo!()
    }

    fn loading(&self) -> bool {
        todo!()
    }

    fn sensitive(&self) -> bool {
        todo!()
    }
}
