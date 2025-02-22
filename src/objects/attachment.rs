use uuid::Uuid;

use crate::Store;

use super::Item;
use crate::objects::{BaseTrait, DueDate};
use crate::schema::attachments;
use crate::utils::EMPTY_DATETIME;
use crate::{Database, Project};
use diesel::QueryDsl;
use diesel::Queryable;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
#[derive(QueryableByName, Queryable, PartialEq, Clone, Eq, Selectable, Serialize, Debug)]
#[diesel(table_name = attachments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Attachment {
    id: Option<String>,
    item_id: Option<String>,
    file_type: Option<String>,
    file_name: Option<String>,
    file_size: Option<String>,
    file_path: Option<String>,
}

impl Attachment {
    pub fn new(
        file_type: Option<String>,
        file_name: Option<String>,
        file_size: Option<String>,
        file_path: Option<String>,
    ) -> Attachment {
        Self {
            id: Some(Uuid::new_v4().to_string()),
            item_id: Some("".to_string()),
            file_type,
            file_name,
            file_size,
            file_path,
        }
    }
    pub fn delete(&self) {
        Store::instance().delete_attachment(self.clone());
    }
    pub fn item(&self) -> Item {
        Store::instance()
            .get_item(self.item_id.clone().unwrap())
            .unwrap()
    }
    pub fn set_item(&mut self, new_item_id: String) {
        self.item_id = Some(new_item_id);
    }
    pub fn to_string(&self) -> String {
        format!(
            "_________________________________\nID: {}\nITEM ID: {}\nFILE TYPE: {}\nFILE NAME: {}\nFILE SIZE: {}\nFILE PATH: {}\n---------------------------------",
            self.id.clone().unwrap(),
            self.item_id.clone().unwrap(),
            self.file_type.clone().unwrap(),
            self.file_name.clone().unwrap(),
            self.file_size.clone().unwrap(),
            self.file_path.clone().unwrap(),
        )
    }
    pub fn duplicate(&self) -> Attachment {
        let new = Attachment::new(
            self.file_type.clone(),
            self.file_name.clone(),
            self.file_size.clone(),
            self.file_path.clone(),
        );
        new
    }
}
