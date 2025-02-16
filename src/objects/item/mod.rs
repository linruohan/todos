mod imp;
pub(crate) use imp::Item;

use super::Project;
use super::{
    Database,
    schema::items::{self, id},
};
use diesel::QueryDsl;
use items::table;
use std::ops::Deref;

pub fn get_item(db: Database, item_id: String) -> Item {
    let mut conn = db.get_conn();
    items::table
        .filter(id.eq(item_id))
        .first::<Item>(&mut conn)
        .map_err(|e| anyhow::anyhow!(e))
        .expect("Error getting")
}
