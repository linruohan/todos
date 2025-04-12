use crate::BaseTrait;
use crate::Source;
use crate::Store;
use crate::enums::SourceType;
use crate::schema::projects;
use diesel::QueryDsl;
use diesel::Queryable;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
#[derive(
    QueryableByName, Queryable, PartialEq, Insertable, Clone, Eq, Selectable, Serialize, Debug,
)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Project {
    pub id: Option<String>,
    pub parent_id: Option<String>,
    pub name: String,
    pub source_id: Option<String>,
    pub color: Option<String>,
    pub backend_type: Option<String>,
    pub inbox_project: Option<i32>,
    pub team_inbox: Option<i32>,
    pub child_order: Option<i32>,
    pub is_deleted: Option<i32>,
    pub is_archived: Option<i32>,
    pub is_favorite: Option<i32>,
    pub shared: Option<i32>,
    pub view_style: Option<String>,
    pub sort_order: Option<i32>,
    pub collapsed: Option<i32>,
    pub icon_style: Option<String>,
    pub emoji: Option<String>,
    pub show_completed: Option<i32>,
    pub description: Option<String>,
    pub due_date: Option<String>,
    pub inbox_section_hidded: Option<i32>,
    pub sync_id: Option<String>,
}

impl Project {
    pub fn source(&self) -> Source {
        Store::instance()
            .get_source(self.source_id.clone().unwrap())
            .unwrap_or(Source::default())
    }
    pub fn source_type(&self) -> SourceType {
        self.source().source_type()
    }
    pub fn parent(&self) -> Option<Project> {
        if let Some(parent_id) = &self.parent_id {
            Store::instance().get_project(parent_id.clone())
        } else {
            None
        }
    }
    pub fn add_subproject(&self, subproject: Project) {
        Store::instance().insert_project(subproject.clone());
    }
}
impl BaseTrait for Project {
    fn source(&self) -> Source {
        self.source()
    }
    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}
