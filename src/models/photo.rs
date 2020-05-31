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

use crate::schema::topic::topic::{self, dsl::*};
use crate::schema::topic::topic_attributes::{self, dsl::*};
use crate::models::topic::{Topic, TopicAttributes};

use diesel::prelude::*;
use diesel::dsl::{count};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhotoItem {
    pub id      : i32,
    pub user_id : String,
    pub url     : String,
    pub tag     : Option<String>,
    pub description : Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PhotoResponse {
    #[serde(rename="totalNum")]
    pub total_num: i64,
    pub rows     : Vec<PhotoItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ApproveStatus {
    Approved,
    Reject,
    Unapproved,
}
impl ApproveStatus {
    pub fn get(&self) -> i32 {
        match *self {
            ApproveStatus::Approved => {
                return 1i32;
            },
            ApproveStatus::Reject => {
                return 2i32;
            },
            ApproveStatus::Unapproved => {
                return 3i32;
            },
        }
    }

    pub fn from(status: i32) -> Self {
        match status {
            1 => {
                return ApproveStatus::Approved;
            },
            2 => {
                return ApproveStatus::Reject;
            },
            3 => {
                return ApproveStatus::Unapproved;
            },
            _ => {
                return ApproveStatus::Unapproved;
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApproveStatusJson {
    pub photo_id: i32,
    pub approve_status  : i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApproveStatusRequest {
    pub photo_id: i32,
    pub approve_status  : ApproveStatus,
}
impl ApproveStatusRequest {
    pub fn new(photo_id: i32, status: ApproveStatus) -> Self {
        ApproveStatusRequest {
            photo_id: photo_id,
            approve_status: status,
        }
    }
}
impl Default for ApproveStatusRequest {
    fn default() -> Self {
        ApproveStatusRequest {
            photo_id: -1,
            approve_status: ApproveStatus::Unapproved,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApproveStatusResponse {
    pub code    : u32,
    pub message : String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeletePhotoResponse {
    pub code    : u32,
    pub message : String,
}

pub async fn get_photos_by_approve_status(status: ApproveStatus, start: u32, step: u32) -> Result<PhotoResponse, ServiceError> {
    let conn = &DBPOOL.get().unwrap();

    let offset = (start - 1) as u32 * step;

    joinable!(topic_attributes -> topic(id));
    allow_tables_to_appear_in_same_query!(topic_attributes, topic);

    let join = topic.inner_join(topic_attributes);

    let mut total_num = 0;
    if let Ok(c) = join.select(count(topic::id)).filter(topic_attributes::approve_status.eq(status.get())).first::<i64>(conn) {
        total_num = c;
    }

    match join.select((topic::id, topic::user_id, topic::url, topic::tag, topic::description))
            .filter(topic_attributes::approve_status.eq(status.get()))
            .order(topic::id)
            .limit(step as i64)
            .offset(offset as i64)        
            .load::<Topic>(conn) {
                Ok(result) => { 
                    let mut v: Vec<PhotoItem> = vec![];
                    for x in &result {                
                        let item: PhotoItem = PhotoItem {
                            id: x.id,
                            user_id: x.user_id.clone(),
                            url: x.url.clone(),
                            tag: x.tag.clone(),
                            description: x.description.clone(),
                        };
        
                        v.push(item);
                    }
                    return Ok(
                        PhotoResponse {
                            total_num: total_num,
                            rows: v
                        });
                },
                Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
            }
}

pub async fn get_all_photos(start: u32, step: u32) -> Result<PhotoResponse, ServiceError> {
    let conn = &DBPOOL.get().unwrap();

    let offset = (start - 1) as u32 * step;

    let mut total_num = 0;
    match topic.select(count(topic::id)).first::<i64>(conn) {
        Ok(c) => {
            total_num = c;
        },
        Err(_) => {},
    }
    
    match topic.select((topic::id, user_id, url, tag, description))
                .order(topic::id)
                .limit(step as i64)
                .offset(offset as i64)
                .load::<Topic>(conn) {
        Ok(result) => { 
            let mut v: Vec<PhotoItem> = vec![];
            for x in &result {                
                let item: PhotoItem = PhotoItem {
                    id          : x.id,
                    user_id     : x.user_id.clone(),
                    url         : x.url.clone(),
                    tag         : x.tag.clone(),
                    description : x.description.clone(),
                };

                v.push(item);
            }
            return Ok(
                PhotoResponse {
                    total_num: total_num,
                    rows: v
                });
        },
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
    }
}

pub async fn update_approve_status(info: ApproveStatusRequest) -> Result<ApproveStatusResponse, ServiceError> {
    let conn = &DBPOOL.get().unwrap();

    let photo_id: i32 = info.photo_id;
    let status: ApproveStatus = info.approve_status;

    match topic_attributes.filter(topic_attributes::id.eq(photo_id)).get_result::<TopicAttributes>(conn) {
        Ok(_) => { 
            match diesel::update(topic_attributes.filter(topic_attributes::id.eq(photo_id))).set(topic_attributes::approve_status.eq(status.get())).execute(conn) {
                Ok(_) => { return Ok(ApproveStatusResponse{ code: 200, message: "Update Success".to_string() }) },
                Err(_) => {
                    return Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()));
                }
            }
        },
        Err(_) => {    
            match diesel::insert_into(topic_attributes).values((topic_attributes::id.eq(photo_id), topic_attributes::approve_status.eq(status.get()))).execute(conn) {
                Ok(_) => {
                    return Ok(ApproveStatusResponse{ code: 200, message: "Update Success".to_string() });
                },
                Err(_) => {
                    return Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()));
                }
            }    
        }
    }
}

pub async fn delete_photo(photo_id: i32) -> Result<DeletePhotoResponse, ServiceError> {
    let conn = &DBPOOL.get().unwrap();

    match topic_attributes.filter(topic_attributes::id.eq(photo_id)).get_result::<TopicAttributes>(conn) {
        Ok(_) => { 
            match diesel::update(topic_attributes.filter(topic_attributes::id.eq(photo_id))).set(topic_attributes::delete_by_admin.eq(true)).execute(conn) {
                Ok(_) => { return Ok(DeletePhotoResponse{ code: 200, message: "Delete Success".to_string() }) },
                Err(_) => {
                    return Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()));
                }
            }
        },
        Err(_) => {    
            match diesel::insert_into(topic_attributes).values((topic_attributes::id.eq(photo_id), topic_attributes::delete_by_admin.eq(true))).execute(conn) {
                Ok(_) => {
                    return Ok(DeletePhotoResponse{ code: 200, message: "Delete Success".to_string() });
                },
                Err(_) => {
                    return Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()));
                }
            }    
        }
    }
}