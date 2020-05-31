table! {
    photo_downloads (non_standard_primary_key) {
        non_standard_primary_key -> Integer,
        photo_id -> Varchar,
        user_id -> Varchar,
        downloads -> Int4,
    }
}

#[derive(Queryable, QueryableByName, Serialize, Deserialize, Debug)]
#[table_name = "photo_downloads"]
pub struct PhotoDownloads {
    pub photo_id: String,
    pub user_id : String,
    pub downloads : u32,
}

