use diesel::prelude::*;
use serde::{Deserialize, Serialize};
mod imp;
use crate::schema::sources;

#[derive(
    QueryableByName, Queryable, Insertable, AsChangeset, Selectable, Serialize, Deserialize, Debug,
)]
#[diesel(table_name = sources)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Sources {
    pub id: Option<String>,
    pub source_type: String,
    pub display_name: Option<String>,
    pub child_order: Option<i32>,
    pub data: Option<String>,
}
