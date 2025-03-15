use crate::BaseObject;

pub mod imp;
pub(crate) use imp::Reminder;
pub struct Reminders {
    base: BaseObject,
    reminder: Reminder,
}
impl Reminders {
    pub(crate) fn source() -> super::Source {
        todo!()
    }
}
