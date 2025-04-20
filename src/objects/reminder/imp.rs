use crate::Item;
use crate::Source;
use crate::Store;
use crate::objects::BaseTrait;
use crate::schema::reminders;
use diesel::Queryable;
use diesel::prelude::*;
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
impl Reminder {
    pub fn item(&self) -> Option<Item> {
        self.item_id
            .as_ref()
            .and_then(|id| Store::instance().get_item(id))
    }
    fn source(&self) -> Option<Source> {
        self.item()
            .and_then(|i| i.project().and_then(|p| p.source()))
    }
}
impl BaseTrait for Reminder {
    fn id(&self) -> &str {
        self.id.as_deref().unwrap_or_default()
    }

    fn set_id(&mut self, id: &str) {
        self.id = Some(id.into());
    }
}
