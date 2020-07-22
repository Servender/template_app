table! {
    pages (id) {
        id -> Int4,
        route_name -> Text,
        page_name -> Text,
        description -> Nullable<Text>,
        path -> Text,
    }
}

table! {
    routes (id) {
        id -> Int4,
        name -> Text,
        route -> Text,
        description -> Nullable<Text>,
        publication -> Int2,
    }
}

table! {
    statics (id) {
        id -> Int4,
        page_id -> Int4,
        name -> Nullable<Text>,
        type_file -> Text,
        status -> Int2,
        mask -> Text,
    }
}

joinable!(statics -> pages (page_id));

allow_tables_to_appear_in_same_query!(
    pages,
    routes,
    statics,
);
