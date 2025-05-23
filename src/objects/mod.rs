pub mod base;
pub mod base_object;
pub mod filters;

pub mod attachment;
pub mod color;
pub mod database;
pub mod queue;
pub mod schema;

pub mod due_date;
pub mod item;
pub mod label;
pub mod project;
pub mod reminder;
pub mod section;
pub mod source;

use std::sync::Arc;

pub(crate) use attachment::Attachment;
pub(crate) use base::*;
pub(crate) use base_object::BaseObject;
pub(crate) use color::Color;
pub(crate) use database::Database;
pub(crate) use due_date::DueDate;
pub(crate) use filters::FilterItem;
pub(crate) use item::Item;
pub(crate) use label::Label;
pub(crate) use project::Project;
pub(crate) use queue::Queue;
pub(crate) use reminder::Reminder;
pub(crate) use section::Section;
pub(crate) use source::Source;

// #[derive(Debug, Clone)]
// pub enum BaseObjectEnum {
//     Project(Arc<Project>),
//     Section(Arc<Section>),
//     Reminder(Arc<Reminder>),
//     Label(Arc<Label>),
//     Item(Arc<Item>),
//     Source(Arc<Source>),
// }
