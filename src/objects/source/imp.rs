use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{enums::SourceType, objects::BaseTrait, schema::sources};

#[derive(QueryableByName, Queryable, Insertable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = sources)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Source {
    pub id: Option<String>,
    pub source_type: String,
    pub display_name: Option<String>,
    pub added_at: Option<String>,
    pub updated_at: Option<String>,
    pub is_visible: Option<i32>,
    pub child_order: Option<i32>,
    pub sync_server: Option<i32>,
    pub last_sync: Option<String>,
    pub data: Option<String>,
}

impl Source {
    pub fn default() -> Source {
        Self {
            id: None,
            source_type: "".to_string(),
            display_name: todo!(),
            added_at: todo!(),
            updated_at: todo!(),
            is_visible: todo!(),
            child_order: todo!(),
            sync_server: todo!(),
            last_sync: todo!(),
            data: todo!(),
        }
    }
    pub fn source_type(&self) -> SourceType {
        SourceType::parse(Some(&self.source_type))
    }
}
impl BaseTrait for Source {
    fn source(&self) -> Source {
        todo!()
    }
    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}
