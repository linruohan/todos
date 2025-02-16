// @generated automatically by Diesel CLI.

diesel::table! {
    attachments (id) {
        id -> Integer,
        item_id -> Text,
        file_type -> Text,
        file_name -> Text,
        file_size -> Text,
        file_path -> Text,
    }
}

diesel::table! {
    cur_temp_ids (id) {
        id -> Integer,
        temp_id -> Text,
        object -> Text,
    }
}

diesel::table! {
    items (id) {
        id -> Integer,
        content -> Text,
        description -> Text,
        due -> Text,
        added_at -> Text,
        completed_at -> Text,
        updated_at -> Text,
        section_id -> Text,
        project_id -> Text,
        parent_id -> Text,
        priority -> Integer,
        child_order -> Integer,
        checked -> Integer,
        is_deleted -> Integer,
        day_order -> Integer,
        collapsed -> Integer,
        pinned -> Integer,
        labels -> Text,
        extra_data -> Text,
        item_type -> Text,
    }
}

diesel::table! {
    labels (id) {
        id -> Integer,
        name -> Text,
        color -> Text,
        item_order -> Nullable<Integer>,
        is_deleted -> Nullable<Integer>,
        is_favorite -> Nullable<Integer>,
        backend_type -> Text,
        source_id -> Text,
    }
}

diesel::table! {
    o_events (id) {
        id -> Nullable<Integer>,
        event_type -> Text,
        event_date -> Nullable<Timestamp>,
        object_id -> Text,
        object_type -> Text,
        object_key -> Text,
        object_old_value -> Text,
        object_new_value -> Text,
        parent_item_id -> Text,
        parent_project_id -> Text,
    }
}

diesel::table! {
    projects (id) {
        id -> Integer,
        name -> Text,
        color -> Text,
        backend_type -> Text,
        inbox_project -> Nullable<Integer>,
        team_inbox -> Nullable<Integer>,
        child_order -> Nullable<Integer>,
        is_deleted -> Nullable<Integer>,
        is_archived -> Nullable<Integer>,
        is_favorite -> Nullable<Integer>,
        shared -> Nullable<Integer>,
        view_style -> Text,
        sort_order -> Nullable<Integer>,
        parent_id -> Text,
        collapsed -> Nullable<Integer>,
        icon_style -> Text,
        emoji -> Text,
        show_completed -> Nullable<Integer>,
        description -> Text,
        due_date -> Text,
        inbox_section_hidded -> Nullable<Integer>,
        sync_id -> Text,
        source_id -> Text,
    }
}

diesel::table! {
    queue (uuid) {
        uuid -> Text,
        object_id -> Text,
        query -> Text,
        temp_id -> Text,
        args -> Text,
        date_added -> Text,
    }
}

diesel::table! {
    reminders (id) {
        id -> Integer,
        notify_uid -> Nullable<Integer>,
        item_id -> Text,
        service -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        due -> Text,
        mm_offset -> Nullable<Integer>,
        is_deleted -> Nullable<Integer>,
    }
}

diesel::table! {
    sections (id) {
        id -> Integer,
        name -> Text,
        archived_at -> Text,
        added_at -> Text,
        project_id -> Text,
        section_order -> Nullable<Integer>,
        collapsed -> Nullable<Integer>,
        is_deleted -> Nullable<Integer>,
        is_archived -> Nullable<Integer>,
        color -> Text,
        description -> Text,
        hidded -> Nullable<Integer>,
    }
}

diesel::table! {
    sources (id) {
        id -> Integer,
        source_type -> Text,
        display_name -> Text,
        added_at -> Text,
        updated_at -> Text,
        is_visible -> Nullable<Integer>,
        child_order -> Nullable<Integer>,
        sync_server -> Nullable<Integer>,
        last_sync -> Text,
        data -> Text,
    }
}

diesel::joinable!(attachments -> items (item_id));
diesel::joinable!(reminders -> items (item_id));
diesel::joinable!(sections -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(
    attachments,
    cur_temp_ids,
    items,
    labels,
    o_events,
    projects,
    queue,
    reminders,
    sections,
    sources,
);
