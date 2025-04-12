use super::Item;
use crate::objects::{BaseTrait, DueDate};
use crate::schema::queue;
use crate::utils::EMPTY_DATETIME;
use crate::{Database, Project};
use diesel::QueryDsl;
use diesel::Queryable;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
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
