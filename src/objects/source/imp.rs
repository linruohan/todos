use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{objects::BaseTrait, schema::sources};

#[derive(QueryableByName, Queryable, Insertable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = sources)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Source {
    pub id: Option<String>,
}

impl Source {
    pub fn default() -> Source {
        Self { id: todo!() }
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
