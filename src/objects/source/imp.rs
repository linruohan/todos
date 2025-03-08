use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{enums::SourceType, objects::BaseTrait, schema::sources};

#[derive(QueryableByName, Queryable, Insertable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = sources)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Source {
    pub id: Option<String>,
    pub source_type: Option<String>,
}

impl Source {
    pub fn default() -> Source {
        Self { id: todo!() }
    }
    pub fn source_type(&self) -> SourceType {
        match self.source_type {
            Some(ref st) => SourceType::parse(Some(st)),
            None => SourceType::NONE,
        }
    }
}
impl BaseTrait for Source {
    fn id(&self) -> String {
        self.id.clone().unwrap()
    }

    fn source(&self) -> Source {
        todo!()
    }

    fn filters(&self) -> std::collections::HashMap<String, crate::objects::FilterItem> {
        todo!()
    }
}
