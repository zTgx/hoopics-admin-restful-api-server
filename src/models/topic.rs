use crate::schema::topic::topic::{self};
use crate::schema::topic::topic_attributes::{self};

#[derive(Queryable, QueryableByName, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "topic"]
pub struct Topic {
    pub id          : i32,
    pub user_id     : String,
    pub url         : String,
    pub tag         : Option<String>,
    pub description : Option<String>,
}

#[derive(Queryable, QueryableByName, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "topic_attributes"]
pub struct TopicAttributes {
    pub id              : i32,
    pub is_private      : bool,
    pub is_deleted      : bool,
    pub approve_status  : i32,
    pub delete_by_admin : bool,
}
