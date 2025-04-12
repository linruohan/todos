use crate::objects::BaseTrait;
use crate::schema::sections;
use derive_builder::Builder;
use diesel::prelude::*;
use diesel::Queryable;
use serde::{Deserialize, Serialize};
#[derive(
    QueryableByName,
    Builder,
    Clone,
    Queryable,
    Insertable,
    PartialEq,
    Eq,
    Selectable,
    Serialize,
    Debug,
)]
#[diesel(table_name = sections)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Section {
    pub id: Option<String>,
    pub name: Option<String>,
    pub archived_at: Option<String>,
    pub added_at: Option<String>,
    pub project_id: Option<String>,
    pub section_order: Option<i32>,
    pub collapsed: Option<i32>,
    pub is_deleted: Option<i32>,
    pub is_archived: Option<i32>,
    pub color: Option<String>,
    pub description: Option<String>,
    pub hidded: Option<i32>,
}

impl Section {
    pub fn is_archived(&self) -> bool {
        self.is_archived.unwrap_or(0) > 0
    }
    pub(crate) fn update_count(&self) {
        todo!()
    }
    pub fn was_archived(&self) -> bool {
        self.project()
            .map(|s| s.was_archived())
            .or_else(|| self.is_archived())
    }
}

impl Section {}
