mod imp;
use std::ops::Deref;

use crate::objects::BaseObject;
pub(crate) use imp::Project;
pub struct ProjectLogic {
    pub project: Project,
    pub base: BaseObject,
}
impl Deref for ProjectLogic {
    type Target = Project;

    fn deref(&self) -> &Self::Target {
        &self.project
    }
}

impl ProjectLogic {}
