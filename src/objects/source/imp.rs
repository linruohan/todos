use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::sources;

#[derive(QueryableByName, Queryable, Insertable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = sources)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Source {
    pub id: String,
}

impl Source {
    pub fn default() -> Source {
        Self { id: todo!() }
    }
}
