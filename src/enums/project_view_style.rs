use serde::Serialize;

#[derive(Serialize)]
pub enum ProjectViewStyle {
    PROGRESS,
    EMOJI,
}
impl ProjectViewStyle {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap().to_lowercase()
    }
    pub fn parse(&self, value: Option<&str>) -> ProjectViewStyle {
        match value {
            Some("progress") => ProjectViewStyle::PROGRESS,
            Some("emoji") => ProjectViewStyle::EMOJI,
            _ => ProjectViewStyle::PROGRESS,
        }
    }
}
