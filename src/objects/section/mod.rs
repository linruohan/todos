mod imp;
use super::{BaseObject, Project};
pub(crate) use imp::Section;

pub struct Sections {
    section: Section,
    base: BaseObject,
}
impl Sections {
    pub(crate) fn project(&self) -> Project {
        todo!()
    }
}
