use actix_web::http;
use diesel::prelude::*;
use serde::{ Deserialize, Serialize };

use crate::service::error::ServiceError;
use crate::constants::message::{
    MESSAGE_DASHBOARD_FAILED_USERS_GET,
    MESSAGE_DASHBOARD_FAILED_PHOTOS_GET,
    MESSAGE_DASHBOARD_FAILED_DOWNLOADS_GET,
};

use crate::config::db::{DBPOOL, STATISTICS_DBPOOL};
use crate::schema::user::user::{dsl::*};
use crate::schema::topic::topic::{dsl::*};
use crate::schema::dashboard::photo_downloads::{self, dsl::*};

#[derive(Serialize, Deserialize, Debug)]
pub struct GeneralData {
    pub user_count  : Option<UserCountResponse>,
    pub photo_count : Option<PhotoCountResponse>,
    pub downloads   : Option<PhotoDownloadsResponse>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCountResponse {
    pub count   : u32,
    pub sub_type: String,
}
pub async fn get_user_count() -> Result<UserCountResponse, ServiceError> {
    let conn = &DBPOOL.get().unwrap();

    match user.count().load::<i64>(conn) {
        Ok(counts) => Ok(
                        UserCountResponse { 
                            count: counts[0] as u32, 
                            sub_type: "user_count".to_string(),
                        }
                    ),
        Err(_) => Err(ServiceError::new(http::StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_DASHBOARD_FAILED_USERS_GET.to_string())),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PhotoCountResponse {
    pub count   : u32,
    pub sub_type: String,
}
pub async fn get_photo_count() -> Result<PhotoCountResponse, ServiceError> {
    let conn = &DBPOOL.get().unwrap();

    match topic.count().load::<i64>(conn) {
        Ok(counts) => Ok(
                        PhotoCountResponse { 
                            count   : counts[0] as u32, 
                            sub_type: "photo_count".to_string(),
                        }
                    ),
        Err(_) => Err(ServiceError::new(http::StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_DASHBOARD_FAILED_PHOTOS_GET.to_string())),
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PhotoDownloadsResponse {
    pub count   : u32,
    pub sub_type: String,
}
pub async fn get_photo_downloads() -> Result<PhotoDownloadsResponse, ServiceError> {
    let conn = &STATISTICS_DBPOOL.get().unwrap();

    match photo_downloads.select(diesel::dsl::sum(photo_downloads::downloads)).load::<Option<i64>>(conn) {
        Ok(counts) => Ok(
                        PhotoDownloadsResponse { 
                            count   : counts[0].unwrap() as u32, 
                            sub_type: "photo_downloads".to_string(),
                        }
                    ),
        Err(_) => Err(ServiceError::new(http::StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_DASHBOARD_FAILED_DOWNLOADS_GET.to_string())),
    }
}