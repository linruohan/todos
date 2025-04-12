use crate::BaseTrait;
use crate::Source;
use crate::Store;
use crate::enums::SourceType;
use crate::schema::reminders;
use diesel::QueryDsl;
use diesel::Queryable;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
#[derive(
    QueryableByName,
    Queryable,
    PartialEq,
    Insertable,
    Clone,
    Eq,
    Selectable,
    Serialize,
    Debug,
    Identifiable,
)]
#[diesel(table_name = reminders)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Reminder {
    pub id: Option<String>,
    pub item_id: Option<String>,
}
impl Reminder {}
