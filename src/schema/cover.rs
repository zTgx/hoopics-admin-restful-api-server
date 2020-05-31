table! {
    cover (id) {
        id       -> Int4,
        photo_id -> Varchar,
    }
}

#[derive(Queryable, QueryableByName, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "cover"]
pub struct Cover {
    pub id : i32,
    pub photo_id: String,
}
