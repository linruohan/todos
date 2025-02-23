use crate::objects::{BaseObject, BaseTrait, DueDate};
use crate::schema::items;
use crate::utils::EMPTY_DATETIME;
use crate::{Attachment, Database, Project};
use diesel::QueryDsl;
use diesel::Queryable;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
#[derive(QueryableByName, Queryable, PartialEq, Eq, Selectable, Serialize, Debug)]
#[diesel(table_name = items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Item {
    pub id: Option<String>,
    pub content: String,
    pub description: Option<String>,
    pub due: Option<String>,
    pub added_at: Option<String>,
    pub completed_at: Option<String>,
    pub updated_at: Option<String>,
    pub section_id: Option<String>,
    pub project_id: Option<String>,
    pub parent_id: Option<String>,
    pub priority: Option<i32>,
    pub child_order: Option<i32>,
    pub checked: Option<i32>,
    pub is_deleted: Option<i32>,
    pub day_order: Option<i32>,
    pub collapsed: Option<i32>,
    pub pinned: Option<i32>,
    pub labels: Option<String>,
    pub extra_data: Option<String>,
    pub item_type: Option<String>,
}
impl Item {
    pub(crate) fn project(&self) -> Project {
        todo!()
    }
    pub fn due(&self) -> DueDate {
        match &self.due {
            Some(due) => serde_json::from_str::<DueDate>(&due).expect("failed to convert due date"),
            None => DueDate::default(),
        }
    }
    pub fn set_due(&mut self, due: DueDate) {
        self.due = Some(serde_json::to_string(&due).expect("failed to convert due date"));
    }
    pub fn has_due(&self) -> bool {
        self.due().datetime() == EMPTY_DATETIME
    }
}

impl BaseTrait for Item {
    fn source(&self) -> crate::Source {
        todo!()
    }

    fn filters(&self) -> std::collections::HashMap<String, crate::objects::FilterItem> {
        todo!()
    }

    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}

pub struct ItemExtra {
    item: Item,
    base: BaseObject,
}
