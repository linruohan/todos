use crate::enums::ItemType;
use crate::objects::{BaseObject, BaseTrait, DueDate};
use crate::schema::sections;
use crate::utils::{self, EMPTY_DATETIME};
use crate::{Attachment, Database, Label, Project, Store, Util, constants};
use chrono::{Local, NaiveDateTime};
use derive_builder::Builder;
use diesel::QueryDsl;
use diesel::Queryable;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
#[derive(
    QueryableByName,
    Builder,
    Clone,
    Queryable,
    Insertable,
    PartialEq,
    Eq,
    Selectable,
    Serialize,
    Debug,
)]
#[diesel(table_name = sections)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Section {
    pub id: Option<String>,
    pub name: Option<String>,
    pub archived_at: Option<String>,
    pub added_at: Option<String>,
    pub project_id: Option<String>,
    pub section_order: Option<i32>,
    pub collapsed: Option<i32>,
    pub is_deleted: Option<i32>,
    pub is_archived: Option<i32>,
    pub color: Option<String>,
    pub description: Option<String>,
    pub hidded: Option<i32>,
}

impl Section {}
