use crate::schema::queue;
use diesel::prelude::*;
use diesel::Queryable;
use serde::{Deserialize, Serialize};
#[derive(
    QueryableByName, Queryable, PartialEq, Insertable, Clone, Eq, Selectable, Serialize, Debug,
)]
#[diesel(table_name = queue)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Queue {
    pub uuid: Option<String>,
    pub object_id: Option<String>,
    pub temp_id: Option<String>,
    pub query: Option<String>,
    pub args: Option<String>,
    pub date_added: Option<String>,
}
impl Queue {}
