use std::{default, fmt};
use strum::{Display, EnumString};
#[derive(Debug, Clone, PartialEq, EnumString, Default)]
#[strum(serialize_all = "camelCase")]
pub enum FilterItemType {
    #[default]
    PRIORITY = 0,
    LABEL = 1,
    DueDate = 2,
    SECTION = 3,
}
impl FilterItemType {
    pub fn get_title(&self) -> &str {
        match self {
            FilterItemType::PRIORITY => "Priority",
            FilterItemType::LABEL => "Label",
            FilterItemType::DueDate => "Due Date",
            FilterItemType::SECTION => "Section",
        }
    }
    pub fn get_icon(&self) -> &str {
        match self {
            FilterItemType::PRIORITY => "flag-outline-thick-symbolic",
            FilterItemType::LABEL => "tag-outline-symbolic",
            FilterItemType::DueDate => "month-symbolic",
            FilterItemType::SECTION => "arrow3-right-symbolic",
        }
    }
}
impl fmt::Display for FilterItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string().to_lowercase())
    }
}
