use crate::Util;
use crate::enums::SourceType;
use crate::objects::{BaseObject, BaseTrait, DueDate};
use crate::schema::labels;
use crate::utils::EMPTY_DATETIME;
use crate::{Attachment, Database, Project, Source, Store};
use derive_builder::Builder;
use diesel::QueryDsl;

use diesel::Queryable;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
#[derive(
    QueryableByName,
    Builder,
    Default,
    Queryable,
    PartialEq,
    Insertable,
    Eq,
    Selectable,
    Deserialize,
    Serialize,
    Debug,
    Clone,
)]
#[diesel(table_name = labels)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[builder(default, setter(into, strip_option))]
pub struct Label {
    pub id: Option<String>,
    #[builder(setter(strip_option))]
    pub name: Option<String>,
    #[builder(setter(strip_option))]
    pub color: Option<String>,
    #[builder(setter(strip_option))]
    pub item_order: Option<i32>,
    #[builder(setter(strip_option))]
    pub is_deleted: Option<i32>,
    #[builder(setter(strip_option))]
    pub is_favorite: Option<i32>,
    #[builder(setter(strip_option))]
    pub backend_type: Option<String>,
    #[builder(setter(strip_option))]
    pub source_id: Option<String>,
}
impl Label {
    pub fn color(&self) -> String {
        self.color.clone().unwrap_or("".to_string())
    }
    pub fn item_order(&self) -> i32 {
        self.item_order.clone().unwrap_or(0)
    }
    pub fn is_deleted(&self) -> bool {
        self.is_deleted.map_or(false, |x| x != 0)
    }
    pub fn is_favorite(&self) -> bool {
        self.is_favorite.map_or(false, |x| x != 0)
    }
    pub fn backend_type(&self) -> SourceType {
        match self.backend_type {
            Some(ref s) => SourceType::parse(Some(s)),
            None => SourceType::NONE,
        }
    }
    fn update_label_count(&self) -> usize {
        Store::instance()
            .get_items_by_label(self.clone(), false)
            .len()
    }
    pub fn source_id(&self) -> String {
        self.source_id
            .clone()
            .unwrap_or(SourceType::LOCAL.to_string())
    }
    pub fn source_type(&self) -> SourceType {
        self.source().source_type()
    }

    pub fn label_count(&self) -> usize {
        let mut count = 0;
        count = self.update_label_count();
        count
    }
    pub fn short_name(&self) -> String {
        Util::get_default().get_short_name(self.name.clone().unwrap_or_default(), 0)
    }
    pub fn delete_label(&self) {
        let items = Store::instance().get_items_by_label(self.clone(), false);
        for item in items {
            item.delete_item_label(self.id.clone().unwrap_or_default());
        }
        Store::instance().delete_label(self.clone());
    }
}

impl BaseTrait for Label {
    fn source(&self) -> Source {
        Store::instance()
            .get_source(self.source_id())
            .unwrap_or(Source::default())
    }

    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}
