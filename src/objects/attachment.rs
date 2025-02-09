use serde::Serialize;
use uuid::Uuid;

use super::Item;

#[derive(Debug, Clone, Serialize)]
pub struct Attachment {
    id: String,
    item_id: String,
    file_type: String,
    file_name: String,
    file_size: i64,
    file_path: String,
}

impl Attachment {
    pub fn new(file_type: String, file_name: String, file_size: i64, file_path: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            item_id: "".to_string(),
            file_type,
            file_name,
            file_size,
            file_path,
        }
    }
    pub fn deleted(&self) {}
    pub fn delete(&self) {
        // store.delete_attachment(self)
    }
    pub fn item(&self) -> Item {
        get_item(self.item_id.clone())
    }
    pub fn set_item(&mut self, new_item_id: String) {
        self.item_id = new_item_id
    }
    pub fn to_string(&self) -> String {
        format!(
            "_________________________________\nID: {}\nITEM ID: {}\nFILE TYPE: {}\nFILE NAME: {}\nFILE SIZE: {}\nFILE PATH: {}\n---------------------------------",
            self.id, self.item_id, self.file_type, self.file_name, self.file_size, self.file_path
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

fn get_item(item_id: String) -> Item {
    Item {}
}
