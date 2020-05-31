use actix_web::{web, HttpResponse, Result};
use crate::service::photo as ServicePhoto;
use crate::models::photo as ModelPhoto;
use crate::models::response::ResponseBody;
use crate::constants::message::{SUCCESS, MESSAGE_LOGIN_SUCCESS};

// 待审核的图片接口
pub async fn get_unapproved_photos(info: web::Path<(u32, u32)>) -> Result<HttpResponse> {
    match ServicePhoto::get_unapproved_photos(info.0, info.1).await {
        Ok(photo_res) => Ok(HttpResponse::Ok().json(ResponseBody::new(SUCCESS, MESSAGE_LOGIN_SUCCESS, photo_res))),
        Err(err) => Ok(err.response()),
    }
}

// 审核通过的图片接口
pub async fn get_approved_photos(info: web::Path<(u32, u32)>) -> Result<HttpResponse> {
    match ServicePhoto::get_approved_photos(info.0, info.1).await {
        Ok(photo_res) => Ok(HttpResponse::Ok().json(ResponseBody::new(SUCCESS, MESSAGE_LOGIN_SUCCESS, photo_res))),
        Err(err) => Ok(err.response()),
    }
}

// 审核不通过的图片接口
pub async fn get_reject_photos(info: web::Path<(u32, u32)>) -> Result<HttpResponse> {
    match ServicePhoto::get_reject_photos(info.0, info.1).await {
        Ok(photo_res) => Ok(HttpResponse::Ok().json(ResponseBody::new(SUCCESS, MESSAGE_LOGIN_SUCCESS, photo_res))),
        Err(err) => Ok(err.response()),
    }
}

// 所有的图片
pub async fn get_all_photos(info: web::Path<(u32, u32)>) -> Result<HttpResponse> {
    match ServicePhoto::get_all_photos(info.0, info.1).await {
        Ok(photo_res) => Ok(HttpResponse::Ok().json(ResponseBody::new(SUCCESS, MESSAGE_LOGIN_SUCCESS, photo_res))),
        Err(err) => Ok(err.response()),
    }
}

// 审核图片
// 管理员操作属性: is_approved / delete_by_admin
pub async fn update_approve_status(info: web::Json<ModelPhoto::ApproveStatusJson>) -> Result<HttpResponse> { 
    match ServicePhoto::update_approve_status(info.photo_id, info.approve_status).await {
        Ok(general_res) => Ok(HttpResponse::Ok().json(general_res)),
        Err(err) => Ok(err.response()),
    }
}

// 删除图片
pub async fn delete_photo(path: web::Path<i32>) -> Result<HttpResponse> {
    match ServicePhoto::delete_photo(*path).await {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(err) => Ok(err.response()),
    }
}
