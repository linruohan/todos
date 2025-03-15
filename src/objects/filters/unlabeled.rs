use crate::BaseObject;

pub struct Unlabeled {
    pub base: BaseObject,
}
impl Unlabeled {
    pub fn get_default() -> Unlabeled {
        Self {
            base: BaseObject::new(
                "Unlabeled".to_string(),
                format!("{};{};{}", "no label", "unlabeled", "filters"),
                "tag-outline-remove-symbolic".to_string(),
                "unlabeled-view".to_string(),
            ),
        }
    }
}
