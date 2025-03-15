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
    Eq,
    Selectable,
    Deserialize,
    Serialize,
    Debug,
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
    pub fn source_id(&self) -> String {
        self.source_id
            .clone()
            .unwrap_or(SourceType::LOCAL.to_string())
    }
    pub fn source_type(&self) -> SourceType {
        self.source().source_type()
    }
}

impl BaseTrait for Label {
    fn source(&self) -> Source {
        Store::instance()
            .get_source(self.source_id())
            .unwrap_or(Source::default())
    }

    fn filters(&self) -> std::collections::HashMap<String, crate::objects::FilterItem> {
        todo!()
    }

    fn id(&self) -> Option<&str> {
        todo!()
    }
}
