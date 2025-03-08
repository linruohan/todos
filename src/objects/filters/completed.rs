use std::{any::Any, collections::HashMap};

use uuid::Uuid;

use super::FilterItem;
use crate::{BaseObject, BaseTrait};
pub struct Completed {
    pub base: BaseObject,
}

impl Completed {
    pub fn default() -> Completed {
        Anytime {
            base: BaseObject::new(
                "Completed".to_string(),
                format!("{};{};{}", "completed", "filters", "logbook"),
                "check-round-outline-symbolic".to_string(),
                FilterType::COMPLETED.to_string(),
            ),
        }
    }
    pub fn count{self} ->i32{
        Store::instance().get_items_completed()
    }
}

public class Objects.Filters.Completed : Objects.BaseObject {
    private static Completed? _instance;
    public static Completed get_default () {
        if (_instance == null) {
            _instance = new Completed ();
        }

        return _instance;
    }

    int? _count = null;
    public int count {
        get {
            if (_count == null) {
                _count = Services.Store.instance ().get_items_completed ().size;
            }

            return _count;
        }

        set {
            _count = value;
        }
    }

    public signal void count_updated ();

    construct {
        name = _("Completed");
        keywords = "%s;%s;%s".printf (_("completed"), _("filters"), _("logbook"));
        icon_name = "check-round-outline-symbolic";
        view_id = FilterType.COMPLETED.to_string ();

        Services.Store.instance ().item_added.connect (() => {
            _count = Services.Store.instance ().get_items_completed ().size;
            count_updated ();
        });

        Services.Store.instance ().item_deleted.connect (() => {
            _count = Services.Store.instance ().get_items_completed ().size;
            count_updated ();
        });

        Services.Store.instance ().item_updated.connect (() => {
            _count = Services.Store.instance ().get_items_completed ().size;
            count_updated ();
        });

        Services.Store.instance ().item_archived.connect (() => {
            _count = Services.Store.instance ().get_items_completed ().size;
            count_updated ();
        });

        Services.Store.instance ().item_unarchived.connect (() => {
            _count = Services.Store.instance ().get_items_completed ().size;
            count_updated ();
        });
    }
}
