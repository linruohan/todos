use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum FilterItemType {
    PRIORITY = 0,
    LABEL = 1,
    DueDate = 2,
    SECTION = 3,
}
impl FilterItemType {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap().to_lowercase()
    }
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
