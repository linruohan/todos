use std::{any::Any, collections::HashMap};

use uuid::Uuid;

use super::FilterItem;
use crate::{BaseObject, BaseTrait};
use crate::{Store, enums::FilterType};
pub struct Labels {
    pub base: BaseObject,
}

impl Labels {
    pub fn get_default() -> Labels {
        Self {
            base: BaseObject::new(
                "Labels".to_string(),
                format!("{};{}", "label", "filters"),
                "tag-outline-symbolic".to_string(),
                FilterType::LABELS.to_string(),
            ),
        }
    }
    pub fn count(&self) -> i32 {
        Store::instance().get_items_has_labels().len()
    }
    pub fn count_updated(&self) {

        // Store::instance().label_added.connect (() => {
        //     _count = Store::instance().get_items_has_labels ().size;
        //     count_updated ();
        // });

        // Store::instance().label_deleted.connect (() => {
        //     _count = Store::instance().get_items_has_labels ().size;
        //     count_updated ();
        // });

        // Store::instance().label_updated.connect (() => {
        //     _count = Store::instance().get_items_has_labels ().size;
        //     count_updated ();
        // });

        // Store::instance().item_added.connect (() => {
        //     _count = Store::instance().get_items_has_labels ().size;
        //     count_updated ();
        // });

        // Store::instance().item_deleted.connect (() => {
        //     _count = Store::instance().get_items_has_labels ().size;
        //     count_updated ();
        // });

        // Store::instance().item_archived.connect (() => {
        //     _count = Store::instance().get_items_has_labels ().size;
        //     count_updated ();
        // });

        // Store::instance().item_unarchived.connect ((item) => {
        //     _count = Store::instance().get_items_has_labels ().size;
        //     count_updated ();
        // });

        // Store::instance().item_updated.connect (() => {
        //     _count = Store::instance().get_items_has_labels ().size;
        //     count_updated ();
        // });
    }
}
