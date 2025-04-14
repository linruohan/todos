use crate::enums::SourceType;
use crate::objects::BaseObjectTrait;
use crate::schema::projects;
use crate::Store;
use diesel::prelude::*;
use diesel::row::NamedRow;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

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
    pub(crate) fn is_inbox_project(&self) -> bool {
        todo!()
    }
    pub(crate) fn is_archived(&self) -> bool {
        self.is_archived.unwrap_or(0) > 0
    }
    pub fn source_type(&self) -> SourceType {
        self.source().source_type()
    }
    pub fn parent(&self) -> Project {
        self.parent_id
            .as_deref()
            .and_then(|id| Store::instance().get_project(id))
            .unwrap_or_else(|_| None.into())
    }
    pub fn add_subproject(&self, subproject: &Project) {
        Store::instance().insert_project(&subproject);
    }
}
impl BaseObjectTrait for Project {}
