use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{objects::BaseTrait, schema::sources};

#[derive(QueryableByName, Queryable, Insertable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = sources)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Source {
    pub id: Option<String>,
}

impl Source {
    pub fn default() -> Source {
        Self { id: todo!() }
    }
}
impl BaseTrait for Source {
    fn id(&self) -> String {
        self.id.clone().unwrap()
    }

    fn name(&self) -> String {
        todo!()
    }

    fn keywords(&self) -> String {
        todo!()
    }

    fn icon_name(&self) -> String {
        todo!()
    }

    fn deleted(&self) {
        todo!()
    }

    fn updated(&self, update_id: String) {
        todo!()
    }

    fn archived(&self) {
        todo!()
    }

    fn unarchived(&self) {
        todo!()
    }

    fn loading(&self) -> bool {
        todo!()
    }

    fn loading_change(&self) {
        todo!()
    }

    fn sensitive(&self) -> bool {
        todo!()
    }

    fn sensitive_change(&self) {
        todo!()
    }

    fn source(&self) -> Source {
        todo!()
    }

    fn filters(&self) -> std::collections::HashMap<String, crate::objects::FilterItem> {
        todo!()
    }
}
