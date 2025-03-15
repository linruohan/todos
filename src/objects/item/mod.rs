mod imp;
use super::BaseObject;
use crate::Database;
use crate::schema::items::{self, id};
use anyhow::Error;
use derive_builder::Builder;
use diesel::QueryResult;
use diesel::prelude::*;
pub(crate) use imp::Item;
use items::table;
use std::ops::Deref;
#[derive(Builder)]
pub struct Items {
    pub item: Item,
    pub base: BaseObject,
    #[builder(default = false, setter(into, strip_option))]
    pub activate_name_editable: bool,
    #[builder(default = false, setter(into, strip_option))]
    pub custom_order: bool,
    #[builder(default = false, setter(into, strip_option))]
    pub show_item: bool,
}
impl Items {
    pub fn ics(&self) -> String {
        // Services.Todoist.get_default ().get_string_member_by_object (extra_data, "ics");
        todo!()
    }
    pub fn calendar_data(&self) -> String {
        // Services.Todoist.get_default ().get_string_member_by_object (extra_data, "calendar-data");
        todo!()
    }
    pub fn get_item(item_id: String) -> Item {
        let mut conn = Database::default().get_conn();
        items::table
            .filter(id.eq(item_id))
            .first::<Item>(&mut conn)
            .map_err(|e| anyhow::anyhow!(e))
            .expect("Error getting")
    }
    pub fn delete_item(item_id: String) -> QueryResult<usize> {
        let mut conn = Database::default().get_conn();
        diesel::delete(items::table.filter(id.eq(item_id))).execute(&mut conn)?;
        Ok(0)
    }
}
