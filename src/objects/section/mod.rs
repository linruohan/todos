pub mod imp;
use super::Project;
use imp::Section;

pub struct Sections {
    section: Section,
}
impl Sections {
    pub(crate) fn project(&self) -> Project {
        todo!()
    }
}
