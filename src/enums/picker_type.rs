use serde::Serialize;

#[derive(Serialize)]
pub enum PickerType {
    PROJECTS,
    SECTIONS,
}

impl PickerType {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap().to_lowercase()
    }
}
