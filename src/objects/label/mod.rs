pub mod imp;
pub(crate) use imp::Label;
pub struct Labels {
    label: Label,
}
impl Labels {
    pub(crate) fn source(&self) -> super::Source {}
}
