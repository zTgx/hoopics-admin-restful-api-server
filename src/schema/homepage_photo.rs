table! {
    rcmd_photos (id) {
        id       -> Int4,
        photo_id -> Int4,
        status   -> Bool,
    }
}

#[derive(Queryable, QueryableByName, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "rcmd_photos"]
pub struct RcmdPhotos {
    pub id      : i32,
    pub photo_id: i32,
    pub status  : bool,
}
