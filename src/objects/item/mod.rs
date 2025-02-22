mod imp;
use super::Project;
use crate::Database;
use crate::schema::items::{self, id};
use anyhow::Error;
use diesel::QueryResult;
use diesel::prelude::*;
pub(crate) use imp::Item;
use items::table;
use std::ops::Deref;
impl Item {
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
