use actix_web::{web, HttpResponse, Result};
use crate::service::user as ServiceUser;
use crate::models::user as ModelUser;
use crate::models::response::ResponseBody;
use crate::constants::message::{SUCCESS, MESSAGE_USE_GET_SUCCESS};

pub async fn get_user_by_id(user_id: web::Path<String>) -> Result<HttpResponse> {
    match ServiceUser::get_user_by_id(user_id.into_inner()).await {
        Ok(res) => Ok(HttpResponse::Ok().json(ResponseBody::new(SUCCESS, MESSAGE_USE_GET_SUCCESS, res))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn get_all_users(pagination: web::Path<ModelUser::Pagination>) -> Result<HttpResponse> {
    match ServiceUser::get_all_users(pagination.into_inner()).await {
        Ok(res) => Ok(HttpResponse::Ok().json(ResponseBody::new(SUCCESS, MESSAGE_USE_GET_SUCCESS, res))),
        Err(err) => Ok(err.response()),
    }
}
 
pub async fn add_user_to_blacklist(info: web::Json<ModelUser::MuteRequest>) -> Result<HttpResponse> {
    match ServiceUser::add_user_to_blacklist(info.into_inner()).await {
        Ok(res) => Ok(HttpResponse::Ok().json(ResponseBody::new(SUCCESS, MESSAGE_USE_GET_SUCCESS, res))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn remove_user_from_blacklist(info: web::Json<ModelUser::MuteRequest>) -> Result<HttpResponse> {
    match ServiceUser::remove_user_from_blacklist(info.into_inner()).await {
        Ok(res) => Ok(HttpResponse::Ok().json(ResponseBody::new(SUCCESS, MESSAGE_USE_GET_SUCCESS, res))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn delete_user(info: web::Json<ModelUser::DeleteUserRequest>) -> Result<HttpResponse> {
    match ServiceUser::delete_user(info.into_inner()).await {
        Ok(res) => Ok(HttpResponse::Ok().json(ResponseBody::new(SUCCESS, MESSAGE_USE_GET_SUCCESS, res))),
        Err(err) => Ok(err.response()),
    }
}
