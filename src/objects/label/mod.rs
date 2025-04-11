pub mod imp;
pub(crate) use imp::Label;

use super::BaseObject;
pub struct Labels {
    label: Label,
    base: BaseObject,
}
impl Labels {}
