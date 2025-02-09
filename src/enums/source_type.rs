use serde::Serialize;

#[derive(Serialize)]
pub enum SourceType {
    NONE,
    LOCAL,
    TODOIST,
    GoogleTasks,
    CALDAV,
}
impl SourceType {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap().to_lowercase()
    }

    pub fn parse(value: Option<&str>) -> SourceType {
        match value {
            Some("local") => SourceType::LOCAL,
            Some("todoist") => SourceType::TODOIST,
            Some("google-tasks") => SourceType::GoogleTasks,
            Some("caldav") => SourceType::CALDAV,
            _ => SourceType::NONE,
        }
    }
}
