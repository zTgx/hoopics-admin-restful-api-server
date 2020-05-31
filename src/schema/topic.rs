table! {
    topic (id) {
        id      -> Int4,
        user_id -> Varchar,
        url     -> Varchar,
        tag     -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        // is_approved -> Int4,
        // is_private -> Int4,
    }
}

table! {
    topic_attributes (id) {
        id      -> Int4,
        is_private -> Bool,
        is_deleted -> Bool,
        approve_status -> Int4,
        delete_by_admin -> Bool,
    }
}