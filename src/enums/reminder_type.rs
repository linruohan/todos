use std::fmt;

use strum::{Display, EnumString};
#[derive(Debug, Clone, PartialEq, EnumString)]
#[strum(serialize_all = "camelCase")]
pub enum ReminderType {
    ABSOLUTE,
    RELATIVE,
}

impl fmt::Display for ReminderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string().to_lowercase())
    }
}
