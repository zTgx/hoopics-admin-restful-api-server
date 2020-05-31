table! {
    blacklists(id) {
        id -> Int4,
        user_id -> Varchar,
        status -> Bool,
    }
}

table! {
    user_delete(id) {
        id -> Int4,
        user_id -> Varchar,
        status -> Bool,
    }
}

table! {
    rcmd_collections (id) {
        id -> Int4,
        cid -> Int4,
        status -> Int4,
    }
}