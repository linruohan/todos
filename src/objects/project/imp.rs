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
#[derive(QueryableByName, Queryable, PartialEq, Clone, Eq, Selectable, Serialize, Debug)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Project {
    pub id: Option<String>,
    pub parent_id: Option<String>,
    pub name: String,
    pub source_id: Option<String>,
}

impl Project {
    pub fn source(&self) -> Source {
        Store::instance()
            .get_source(self.source_id.clone().unwrap())
            .unwrap_or(Source::default())
    }
    pub fn source_type(&self) -> SourceType {
        let source_type = self.source().source_type;
        match source_type {
            Some(ref st) => SourceType::parse(Some(st)),
            None => SourceType::NONE,
        }
    }
}
impl BaseTrait for Project {
    fn source(&self) -> Source {
        self.source()
    }

    fn filters(&self) -> std::collections::HashMap<String, crate::objects::FilterItem> {
        self.filters.clone()
    }

    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}
