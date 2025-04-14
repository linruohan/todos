use crate::enums::ObjectType;
use crate::objects::{FilterItem, Item, Label, Project, Reminder, Section, Source};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum BaseObject {
    Project(Arc<Project>),
    Section(Arc<Section>),
    Reminder(Arc<Reminder>),
    Label(Arc<Label>),
    Item(Arc<Item>),
    Source(Arc<Source>),
}
// 基础对象特性
pub trait BaseObjectTrait {
    fn id(&self) -> &str {
        match self {
            BaseObject::Project(p) => &p.id.as_deref().into(),
            BaseObject::Section(s) => &s.id.as_deref().into(),
            BaseObject::Item(i) => &i.id.as_deref().into(),
            BaseObject::Label(l) => &l.id.as_deref().into(),
            BaseObject::Reminder(r) => &r.id.as_deref().into(),
        }
    }
    fn set_id(&mut self, id: &str) {}
    fn name(&self) -> &str {
        ""
    }
    fn set_name(&mut self, name: &str) {}
    fn keywords(&self) -> &str {
        ""
    }
    fn set_keywords(&mut self, keywords: &str) {}
    fn icon_name(&self) -> &str {
        ""
    }
    fn set_icon_name(&mut self, icon_name: &str) {}
    fn update_timeout_id(&self) -> i32 {
        0
    }
    fn set_update_timeout_id(&self, update_timeout_id: i32) {}
    fn id_string(&self) -> String {
        self.id().to_string()
    }

    fn object_type(&self) -> ObjectType {
        match self {
            BaseObject::Project(p) => ObjectType::PROJECT,
            BaseObject::Section(s) => ObjectType::SECTION,
            BaseObject::Item(i) => ObjectType::ITEM,
            BaseObject::Label(l) => ObjectType::LABEL,
            _ => ObjectType::FILTER,
        }
    }

    fn filters(&self) -> &HashMap<String, FilterItem> {
        self.filters().map_or(&HashMap::new(), |v| v)
    }

    fn add_filter(&mut self, filter: FilterItem) {
        if !self.filters().contains_key(&filter.id()) {
            self.filters().insert(filter.id().to_owned(), filter);
        }
    }

    fn remove_filter(&mut self, filter: FilterItem) {
        if self.filters().contains_key(&filter.id()) {
            self.filters().remove(&filter.id());
        }
    }

    fn update_filter(&mut self, filter: FilterItem) {
        let mut id = filter.id();
        if self.filters().contains_key(&id) {
            self.filters()[id] = filter;
        }
    }

    fn get_filter(&self, id: &str) -> Option<&FilterItem> {
        self.filters().iter().find(|f| f.contains_key(id))
    }

    fn view_id(&self) -> &str {
        ""
    }
    fn loading(&self) -> bool {
        false
    }
    fn sensitive(&self) -> bool {
        false
    }

    fn type_delete(&self) -> &'static str {
        match self {
            BaseObject::Project(_) => "project_delete",
            BaseObject::Section(_) => "section_delete",
            BaseObject::Reminder(_) => "reminder_delete",
            BaseObject::Label(_) => "label_delete",
            BaseObject::Item(_) => "item_delete",
            _ => "",
        }
    }

    fn type_add(&self) -> &'static str {
        match self {
            BaseObject::Project(_) => "project_add",
            BaseObject::Section(_) => "section_add",
            BaseObject::Label(_) => "label_add",
            BaseObject::Item(_) => "item_add",
            _ => "",
        }
    }

    fn type_update(&self) -> &'static str {
        match self {
            BaseObject::Project(_) => "project_update",
            BaseObject::Section(_) => "section_update",
            BaseObject::Label(_) => "label_update",
            BaseObject::Item(_) => "item_update",
            _ => "",
        }
    }

    fn object_type_string(&self) -> &'static str {
        match self {
            BaseObject::Project(p) => "project",
            BaseObject::Section(s) => "section",
            BaseObject::Label(l) => "label",
            BaseObject::Item(i) => "item",
            _ => "filter",
        }
    }

    fn table_name(&self) -> &'static str {
        match self {
            BaseObject::Project(_) => "Projects",
            BaseObject::Section(_) => "Sections",
            BaseObject::Label(_) => "Labels",
            BaseObject::Item(_) => "Items",
            _ => "",
        }
    }

    fn column_order_name(&self) -> &'static str {
        match self {
            BaseObject::Project(p) => "child_order",
            BaseObject::Section(s) => "section_order",
            BaseObject::Label(l) => "item_order",
            BaseObject::Item(i) => "child_order",
            _ => "",
        }
    }

    fn get_update_json(&self, uuid: &str, temp_id: Option<&str>) -> &str {
        ""
    }

    fn get_add_json(&self, temp_id: &str, uuid: &str) -> &str {
        ""
    }

    fn get_move_json(&self, uuid: &str, new_project_id: &str) -> &str {
        ""
    }
    fn to_json(&self) -> &str {
        ""
    }
    fn source(&self) -> Source {
        match self {
            BaseObject::Project(p) => p.source(),
            BaseObject::Section(s) => s.project().source(),
            BaseObject::Item(i) => i.project().map_or(Source::default(), |p| p.source()),
            BaseObject::Label(l) => l.source(),
            BaseObject::Reminder(r) => r.item().project().source(),
            _ => Source::default(),
        }
    }
}

pub struct BaseObjectEvents {
    pub deleted: Option<Box<dyn Fn()>>,
    pub updated: Option<Box<dyn Fn(String)>>,
    pub archived: Option<Box<dyn Fn()>>,
    pub unarchived: Option<Box<dyn Fn()>>,
    pub loading_change: Option<Box<dyn Fn()>>,
    pub sensitive_change: Option<Box<dyn Fn()>>,
    pub filter_added: Option<Box<dyn Fn(FilterItem)>>,
    pub filter_removed: Option<Box<dyn Fn(FilterItem)>>,
    pub filter_updated: Option<Box<dyn Fn(FilterItem)>>,
}

impl BaseObjectTrait for BaseObject {}
