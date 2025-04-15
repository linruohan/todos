use crate::objects::BaseObjectTrait;
use crate::schema::reminders;
use diesel::prelude::*;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

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
impl BaseObjectTrait for Reminder {}
