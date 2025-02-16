use std::ops::Deref;

use super::{BaseObject, Project};

pub struct Item {
    base: BaseObject,
}
impl Item {
    pub(crate) fn project(&self) -> Project {
        todo!()
    }
}

impl Deref for Item {
    type Target = BaseObject;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
