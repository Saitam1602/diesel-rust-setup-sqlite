// @generated automatically by Diesel CLI.

diesel::table! {
    note_tags (note_id, tag_id) {
        note_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    notebook_tags (notebook_id, tag_id) {
        notebook_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    notebooks (id) {
        id -> Nullable<Integer>,
        title -> Text,
        status_id -> Nullable<Integer>,
        notebook_id -> Nullable<Integer>,
    }
}

diesel::table! {
    notes (id) {
        id -> Nullable<Integer>,
        title -> Text,
        text -> Text,
        status_id -> Nullable<Integer>,
        notebook_id -> Nullable<Integer>,
    }
}

diesel::table! {
    statuses (id) {
        id -> Integer,
        name -> Text,
        color -> Text,
    }
}

diesel::table! {
    tags (id) {
        id -> Nullable<Integer>,
        name -> Text,
        color -> Text,
    }
}

diesel::joinable!(note_tags -> notes (note_id));
diesel::joinable!(note_tags -> tags (tag_id));
diesel::joinable!(notebook_tags -> notebooks (notebook_id));
diesel::joinable!(notebook_tags -> tags (tag_id));
diesel::joinable!(notebooks -> statuses (status_id));
diesel::joinable!(notes -> notebooks (notebook_id));
diesel::joinable!(notes -> statuses (status_id));

diesel::allow_tables_to_appear_in_same_query!(
    note_tags,
    notebook_tags,
    notebooks,
    notes,
    statuses,
    tags,
);
