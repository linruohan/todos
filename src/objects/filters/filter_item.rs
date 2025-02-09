use crate::enums::FilterItemType;
#[derive(Clone, Debug)]
pub struct FilterItem {
    pub filter_type: FilterItemType,
    pub name: String,
    pub value: String,
}
impl FilterItem {
    pub fn id(&self) -> String {
        match self.filter_type {
            FilterItemType::DueDate | FilterItemType::SECTION => format!("{:?}", self.filter_type),
            _ => format!("{:?}-{}", self.filter_type, self.value),
        }
    }
}
