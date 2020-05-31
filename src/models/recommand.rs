use actix_web::{    
    http::{
        StatusCode,
    },
};
use crate::service::error::ServiceError;
use crate::constants::message::{
    MESSAGE_INTERNAL_SERVER_ERROR,
};
use crate::config::db::DBPOOL;

use crate::schema::cover::cover::{self, dsl::*};
use crate::schema::cover::Cover;

use diesel::prelude::*;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoverRequest {
    pub photo_id  : String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoverResponse {
    pub photo_id: String,
    pub message : String,
}

pub async fn recommand_to_cover(cover_request: CoverRequest) -> Result<CoverResponse, ServiceError> {
    let conn = &DBPOOL.get().unwrap();

    match cover.filter(cover::photo_id.eq(&cover_request.photo_id)).get_result::<Cover>(conn) {
        Ok(_) => { 
            match diesel::update(cover.filter(cover::photo_id.eq(&cover_request.photo_id))).set(cover::photo_id.eq(&cover_request.photo_id)).execute(conn) {
                Ok(_) => { 
                    return Ok(CoverResponse{ photo_id: cover_request.photo_id, message: "推荐到Cover成功".to_string() }) 
                },
                Err(_) => {
                    return Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()));
                }
            }
        },
        Err(_) => {    
            match diesel::insert_into(cover).values(cover::photo_id.eq(&cover_request.photo_id)).execute(conn) {
                Ok(_) => {
                    return Ok(CoverResponse{ photo_id: cover_request.photo_id, message: "推荐到Cover成功".to_string() }) 
                },
                Err(_) => {
                    return Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()));
                }
            } 
        }
    }
}

use crate::schema::homepage_photo::rcmd_photos::{self, dsl::*};
use crate::schema::homepage_photo::RcmdPhotos;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HomepagePhotoRequest {
    pub photo_id  : i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HomepagePhotoResponse {
    pub photo_id: i32,
    pub message : String,
}

pub async fn recommand_photo_to_homepage(homepage_photo_request: HomepagePhotoRequest) -> Result<HomepagePhotoResponse, ServiceError> {
    let conn = &DBPOOL.get().unwrap();

    match rcmd_photos.filter(rcmd_photos::photo_id.eq(homepage_photo_request.photo_id)).get_result::<RcmdPhotos>(conn) {
        Ok(_) => { 
            match diesel::update(rcmd_photos.filter(rcmd_photos::photo_id.eq(homepage_photo_request.photo_id))).set(rcmd_photos::photo_id.eq(homepage_photo_request.photo_id)).execute(conn) {
                Ok(_) => { 
                    return Ok(HomepagePhotoResponse{ photo_id: homepage_photo_request.photo_id, message: "推荐到首页成功".to_string() }) 
                },
                Err(_) => {
                    return Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()));
                }
            }
        },
        Err(_) => {    
            match diesel::insert_into(rcmd_photos).values(rcmd_photos::photo_id.eq(homepage_photo_request.photo_id)).execute(conn) {
                Ok(_) => {
                    return Ok(HomepagePhotoResponse{ photo_id: homepage_photo_request.photo_id, message: "推荐到首页成功".to_string() }) 
                },
                Err(_) => {
                    return Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()));
                }
            } 
        }
    }
}