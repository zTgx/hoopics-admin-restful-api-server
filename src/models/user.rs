use serde::{ Deserialize, Serialize };
use diesel::prelude::*;

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
use crate::schema::user::{
    user::{self, dsl::*},
    user_attributes::{self, dsl::*},
    User,
    UserAttributes,
    user_details::{self},
    UserDetails,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    pub start: i32,
    pub step: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserItem {
    pub id      : i32,
    pub user_id : String,
    pub name    : String,
    pub email   : String,
    pub avatar  : String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    #[serde(rename="totalNum")]
    pub total_num: i64,
    pub rows     : Vec<UserInfo>,
}

#[diesel(embed)]
#[derive(Serialize, Deserialize, Debug, FromSqlRow)]
pub struct UserInfo {
    pub id      : i32,
    pub user_id : String,
    pub name    : String,
    pub email   : String,
    pub avatar  : String,

    pub location    : Option<String>,
    pub interests   : Option<String>,
    pub douban      : Option<String>,
    pub weibo       : Option<String>,
    pub bio         : Option<String>,
}

pub async fn get_user_by_id(uid: String) -> Result<UserInfo, ServiceError> {
    let conn = &DBPOOL.get().unwrap();
    
    joinable!(user -> user_details(user_id));
    allow_tables_to_appear_in_same_query!(user_details, user);

    let join = user::table.left_join(user_details::table);
    match join
    // .select(( user::id, user::name, user::email, user::avatar,
    //                     user_details::location, user_details::interests, user_details::douban, user_details::weibo, user_details::bio))
            .filter(user_details::user_id.eq(uid))
            .first::<(User, Option<UserDetails>)>(conn) {
                Ok(user_info) => {

                    let mut location1 = None;
                    let mut interests1 = None;
                    let mut douban1 = None;
                    let mut weibo1 = None;
                    let mut bio1 = None;
                    if let Some(x) = user_info.1 {
                        location1 = x.location;
                        interests1 = x.interests;
                        douban1 = x.douban;
                        weibo1 = x.weibo;
                        bio1 = x.bio;
                    }
                    
                    Ok(
                        UserInfo {
                            id: user_info.0.id,
                            user_id: user_info.0.user_id,
                            name: user_info.0.name,
                            email: user_info.0.email,
                            avatar: user_info.0.avatar,
                            
                            location : location1,
                            interests: interests1,
                            douban: douban1,
                            weibo: weibo1,
                            bio: bio1,                     
                        }
                    )
                },
                Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
            }
}

pub async fn get_all_users(pagination: Pagination) -> Result<UserResponse, ServiceError> {
    let conn = &DBPOOL.get().unwrap();

    let start = pagination.start;
    let step  = pagination.step;

    let offset = (start - 1) * step;

    let mut total_num = 0;
    if let Ok(c) = user.select(diesel::dsl::count(user::id)).first::<i64>(conn) {
        total_num = c;
    }
    
    let join = user::table.left_join(user_details::table);
    match join
            .limit(step as i64)
            .offset(offset as i64)
            .load::<(User, Option<UserDetails>)>(conn) {
                Ok(all_users) => {
                    let mut v: Vec<UserInfo> = vec![];
                    for user_info in &all_users {      
                        let mut location = None;
                        let mut interests = None;
                        let mut douban = None;
                        let mut weibo = None;
                        let mut bio = None;

                        if let Some(x) = &user_info.1 {
                            location    = x.location.clone();
                            interests   = x.interests.clone();
                            douban      = x.douban.clone();
                            weibo       = x.weibo.clone();
                            bio         = x.bio.clone();
                        }

                        let item: UserInfo = UserInfo {          
                            id      : user_info.0.id.clone(),
                            user_id : user_info.0.user_id.clone(),
                            name    : user_info.0.name.clone(),
                            email   : user_info.0.email.clone(),
                            avatar  : user_info.0.avatar.clone(),
                            
                            location    : location,
                            interests   : interests,
                            douban      : douban,
                            weibo       : weibo,
                            bio         : bio, 
                        };

                        v.push(item);
                    }

                    Ok(
                        UserResponse {
                            total_num: total_num,
                            rows: v
                        }
                    )
                },
                Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
            }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MuteRequest {
    pub user_id: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MuteResponse {
    pub user_id: String,
    pub mute : bool,
}

pub fn add_user_to_blacklist(mute_request: MuteRequest) -> Result<MuteResponse, ServiceError> {
    let conn = &DBPOOL.get().unwrap();

    match user_attributes.filter(user_attributes::id.eq(mute_request.user_id.clone())).get_result::<UserAttributes>(conn) {
        Ok(_) => {
            match diesel::update(user_attributes.filter(user_attributes::id.eq(mute_request.user_id.clone()))).set(user_attributes::mute.eq(true)).execute(conn) {
                Ok(_) => return Ok(MuteResponse{ user_id: mute_request.user_id.clone(), mute: true}),
                Err(_) => {
                    return Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()));
                }
            }
        },
        Err(_) => {            
            match diesel::insert_into(user_attributes).values((user_attributes::id.eq(mute_request.user_id.clone()), user_attributes::mute.eq(true))).execute(conn) {
                Ok(_) => {
                    return Ok(MuteResponse{ user_id: mute_request.user_id.clone(), mute: true});
                },
                Err(_) => {
                    return Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()));
                }
            }
        }    
    }    
}

pub fn remove_user_from_blacklist(info: MuteRequest) -> Result<MuteResponse, ServiceError> {
    let conn = &DBPOOL.get().unwrap();

    match user_attributes.filter(user_attributes::id.eq(&info.user_id)).get_result::<UserAttributes>(conn) {
        Ok(_) => {
            match diesel::update(user_attributes.filter(user_attributes::id.eq(&info.user_id))).set(user_attributes::mute.eq(false)).execute(conn) {
                Ok(_) => return Ok(MuteResponse{ user_id: info.user_id.clone(), mute: false}),
                Err(_) => {
                    return Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()));
                }
            }
        },
        Err(_) => {            
            match diesel::insert_into(user_attributes).values((user_attributes::id.eq(&info.user_id), user_attributes::mute.eq(false))).execute(conn) {
                Ok(_) => {
                    return Ok(MuteResponse{ user_id: info.user_id.clone(), mute: false});
                },
                Err(_) => {
                    return Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()));
                }
            }
        }    
    }    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteUserRequest {
    pub user_id: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteUserResponse {
    pub user_id: String,
    pub status: bool,
}

pub fn delete_user(delete_user_request: DeleteUserRequest) -> Result<DeleteUserResponse, ServiceError> {
    let conn = &DBPOOL.get().unwrap();

    match user_attributes.filter(user_attributes::id.eq(&delete_user_request.user_id)).get_result::<UserAttributes>(conn) {
        Ok(_) => {
            match diesel::update(user_attributes.filter(user_attributes::id.eq(&delete_user_request.user_id))).set(user_attributes::delete_by_admin.eq(true)).execute(conn) {
                Ok(_) => return Ok(DeleteUserResponse{ user_id: delete_user_request.user_id, status: true}),
                Err(_) => {
                    return Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()));
                }
            }
        },
        Err(_) => {            
            match diesel::insert_into(user_attributes).values((user_attributes::id.eq(&delete_user_request.user_id), user_attributes::delete_by_admin.eq(true))).execute(conn) {
                Ok(_) => {
                    return Ok(DeleteUserResponse{ user_id: delete_user_request.user_id, status: true});
                },
                Err(_) => {
                    return Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()));
                }
            }
        }    
    }    
}
