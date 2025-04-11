pub mod imp;
use crate::{BaseObject, BaseTrait, Source};
pub(crate) use imp::Project;
pub struct Projects {
    base: BaseObject,
    project: Project,
}
impl Projects {
    pub fn add_project(&self) -> Project {
        let mut project = self.project.clone();
        project.name = self.base.name.clone();
        project
    }
    pub fn get_project(&self) -> Project {
        self.project.clone()
    }
}
