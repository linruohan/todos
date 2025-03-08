use crate::{BaseObject, Store};

pub struct Priority {
    base: BaseObject,
    priority: i32,
}
impl Priority {
    pub fn get_default(priority: i32) -> Priority {
        let name = Util.get_default().get_priority_title(priority);
        let keywords = format!(
            "{};{}",
            Util.get_default().get_priority_keywords(priority),
            "filters"
        );
        let view_id = format!("priority-{}", priority);
        Self {
            base: BaseObject::new(name, keywords, "".to_string(), view_id),
            priority,
        }
    }
    pub fn count(&self) -> i32 {
        Store::instance()
            .get_items_by_priority(self.priority, false)
            .len()
    }
    pub fn count_updated(&self) {

        // Store::instance().item_added.connect (() => {
        //     _count = Store::instance().get_items_by_priority (priority, false).size;
        //     count_updated ();
        // });

        // Store::instance().item_deleted.connect (() => {
        //     _count = Store::instance().get_items_by_priority (priority, false).size;
        //     count_updated ();
        // });

        // Store::instance().item_updated.connect (() => {
        //     _count = Store::instance().get_items_by_priority (priority, false).size;
        //     count_updated ();
        // });

        // Store::instance().item_archived.connect (() => {
        //     _count = Store::instance().get_items_by_priority (priority, false).size;
        //     count_updated ();
        // });

        // Store::instance().item_unarchived.connect (() => {
        //     _count = Store::instance().get_items_by_priority (priority, false).size;
        //     count_updated ();
        // });
    }
}
