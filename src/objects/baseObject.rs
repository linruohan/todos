use crate::objects::{Item, Label, Project, Reminder, Section};

pub enum BaseObject {
    Project(Project),
    Section(Section),
    Reminder(Reminder),
    Label(Label),
    Item(Item),
}

impl BaseObject {}