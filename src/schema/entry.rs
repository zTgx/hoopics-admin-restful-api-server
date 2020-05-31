table! {
    admin (id) {
        id          -> Int4,
        user_id     -> Varchar,
        name        -> Varchar,
        password    -> Varchar,
        avatar      -> Varchar,
        email       -> Varchar,
        group       -> Int4,
    }
}

#[derive(Queryable, QueryableByName, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "admin"]
pub struct Admin {
    pub id      : i32,
    pub user_id : String,
    pub name    : String,
    pub password: String,
    pub avatar  : String,
    pub email   : String,
    pub group   : i32,
}
