use serde::Serialize;

#[derive(Serialize)]
pub enum ProjectIconStyle {
    LIST,
    BOARD,
}
impl ProjectIconStyle {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap().to_lowercase()
    }
    pub fn parse(&self, value: Option<&str>) -> ProjectIconStyle {
        match value {
            Some("list") => ProjectIconStyle::LIST,
            Some("board") => ProjectIconStyle::BOARD,
            _ => ProjectIconStyle::LIST,
        }
    }
}
