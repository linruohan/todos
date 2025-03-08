pub mod imp;
use crate::{BaseObject, BaseTrait, Source};
pub(crate) use imp::Project;
pub struct Projects {
    base: BaseObject,
    project: imp::Project,
}
impl Projects {
    pub(crate) fn source() -> Source {
        todo!()
    }
}

impl BaseTrait for Projects {
    fn source(&self) -> Source {
        self.project.source()
    }

    fn filters(&self) -> std::collections::HashMap<String, super::FilterItem> {
        todo!()
    }

    fn id(&self) -> Option<&str> {
        todo!()
    }
}
