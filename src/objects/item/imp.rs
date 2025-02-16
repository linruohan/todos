use crate::objects::BaseTrait;
use crate::schema::items;
use crate::{Database, Project};
use diesel::QueryDsl;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
#[derive(QueryableByName, Queryable, Insertable, AsChangeset, Selectable, Serialize, Debug)]
#[diesel(table_name = items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Item {
    pub id: i32,
    pub content: String,
    pub description: String,
    pub due: String,
    pub added_at: String,
    pub completed_at: String,
    pub updated_at: String,
    pub section_id: String,
    pub project_id: String,
    pub parent_id: String,
    pub priority: i32,
    pub child_order: i32,
    pub checked: i32,
    pub is_deleted: i32,
    pub day_order: i32,
    pub collapsed: i32,
    pub pinned: i32,
    pub labels: String,
    pub extra_data: String,
    pub item_type: String,
}
impl Item {
    pub(crate) fn project(&self) -> Project {
        todo!()
    }
}

impl BaseTrait for Item {
    fn id(&self) -> i32 {
        todo!()
    }

    fn name(&self) -> String {
        todo!()
    }

    fn keywords(&self) -> String {
        todo!()
    }

    fn icon_name(&self) -> String {
        todo!()
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

    fn source(&self) -> crate::Source {
        todo!()
    }

    fn filters(&self) -> std::collections::HashMap<String, crate::objects::FilterItem> {
        todo!()
    }
}
