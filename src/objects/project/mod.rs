pub mod imp;
use crate::{BaseObject, BaseTrait, Source};
pub(crate) use imp::Project;
pub struct Projects {
    base: BaseObject,
    project: Project,
}
impl Projects {}
