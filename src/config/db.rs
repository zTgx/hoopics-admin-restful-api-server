use diesel::{
    mysql::MysqlConnection,
    r2d2::{self, ConnectionManager},
};
use std::{env};

pub type Connection = MysqlConnection;
pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

// hoopics_admin数据库
lazy_static! {
    pub static ref ADMINDBPOOL: Pool = {
        let db_url = env::var("HOOPICS_ADMIN_DATABASE_URL").expect("HOOPICS_ADMIN_DATABASE_URL not found.");
        let manager = ConnectionManager::<Connection>::new(db_url);
        let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    
        pool
    };
}

// hoopics数据库
lazy_static! {
    pub static ref DBPOOL: Pool = {
        let db_url = env::var("HOOPICS_DATABASE_URL").expect("HOOPICS_DATABASE_URL not found.");
        let manager = ConnectionManager::<Connection>::new(db_url);
        let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    
        pool
    };
}

// hoopics_statistic数据库
lazy_static! {
    pub static ref STATISTICS_DBPOOL: Pool = {
        let db_url = env::var("HOOPICS_STATISTICS_DATABASE_URL").expect("HOOPICS_STATISTICS_DATABASE_URL not found.");
        let manager = ConnectionManager::<Connection>::new(db_url);
        let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    
        pool
    };
}