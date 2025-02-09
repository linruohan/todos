use serde::Serialize;

#[derive(Serialize)]
pub enum ReminderType {
    ABSOLUTE,
    RELATIVE,
}
impl ReminderType {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap().to_lowercase()
    }
}
