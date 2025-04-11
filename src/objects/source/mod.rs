mod imp;

pub(crate) use imp::Source;

use super::BaseObject;
pub struct Sources {
    base: BaseObject,
    source: Source,
}
impl Sources {
    pub fn add_source(&self) -> Source {
        let mut source = self.source.clone();
        source.name = self.base.name.clone();
        source
    }
    pub fn get_source(&self) -> Source {
        self.source.clone()
    }
}
