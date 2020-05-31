table! {
    user (id) {
        id          -> Int4,
        user_id     -> Varchar,
        name        -> Varchar,
        password    -> Varchar,
        avatar      -> Varchar,
        email       -> Varchar,
    }
}

#[derive(Queryable, QueryableByName, Deserialize, Debug)]
#[table_name = "user"]
pub struct User {
    pub id      : i32,
    pub user_id : String,
    pub name    : String,
    pub password: String,
    pub avatar  : String,
    pub email   : String,
}

table! {
    user_attributes (id) {
        id              -> Varchar,
        is_deleted      -> Bool,
        mute            -> Bool,
        mute_interval   -> Varchar,
        delete_by_admin -> Bool,
    }
}

#[derive(Queryable, QueryableByName, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "user_attributes"]
pub struct UserAttributes {
    pub id              : String,
    pub is_deleted      : bool,
    pub mute            : bool,
    pub mute_interval   : String,
    pub delete_by_admin : bool,
}

table! {
    user_details (user_id) {
        user_id              -> Varchar,
        location             -> Nullable<Varchar>,
        interests            -> Nullable<Varchar>,
        douban               -> Nullable<Varchar>,
        weibo                -> Nullable<Varchar>,
        bio                  -> Nullable<Varchar>,
    }
}

#[derive(Queryable, QueryableByName, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "user_details"]
pub struct UserDetails {
    pub user_id         : String,
    pub location        : Option<String>,
    pub interests       : Option<String>,
    pub douban          : Option<String>,
    pub weibo           : Option<String>,
    pub bio             : Option<String>,
}


