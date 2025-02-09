use serde::Serialize;

#[derive(Serialize)]
pub enum ObjectEventType {
    INSERT,
    UPDATE,
}
impl ObjectEventType {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap().to_lowercase()
    }
    pub fn parse(&self, value: Option<&str>) -> ObjectEventType {
        match value {
            Some("insert") => ObjectEventType::INSERT,
            Some("update") => ObjectEventType::UPDATE,
            _ => ObjectEventType::INSERT,
        }
    }
    pub fn get_label(&self) -> &str {
        match self {
            ObjectEventType::INSERT => "Task Created",
            ObjectEventType::UPDATE => "Task Updated",
        }
    }
}
