use serde::Serialize;

#[derive(Serialize)]
pub enum ItemType {
    TASK,
    NOTE,
}
impl ItemType {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap().to_lowercase()
    }
    pub fn parse(&self, value: Option<&str>) -> ItemType {
        match value {
            Some("task") => return ItemType::TASK,
            Some("note") => return ItemType::NOTE,
            _ => ItemType::TASK,
        }
    }
}
