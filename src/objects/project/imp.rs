use crate::BaseTrait;
use crate::Source;
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
    pub id: Uuid,
    pub name: String,
    pub source: Source,
    pub filters: std::collections::HashMap<String, crate::objects::FilterItem>,
    pub sections: Vec<Section>,
}

impl Project {
    pub fn source(&self) -> Source {
        self.source.clone()
    }
}
impl BaseTrait for Project {
    fn source(&self) -> Source {
        self.source.clone()
    }

    fn filters(&self) -> std::collections::HashMap<String, crate::objects::FilterItem> {
        self.filters.clone()
    }

    fn id(&self) -> Option<&str> {
        self.id.clone()
    }
}
