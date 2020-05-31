use actix_web::{web, HttpResponse, Result};
use crate::service::recommand as ServiceRecommand;
use crate::models::recommand as ModelRecommand;
use crate::models::response::ResponseBody;
use crate::constants::message::{SUCCESS, MESSAGE_LOGIN_SUCCESS};

// 推荐图片到Cover
pub async fn recommand_to_cover(info: web::Path<ModelRecommand::CoverRequest>) -> Result<HttpResponse> {
    match ServiceRecommand::recommand_to_cover(info.into_inner()).await {
        Ok(cover) => Ok(HttpResponse::Ok().json(ResponseBody::new(SUCCESS, MESSAGE_LOGIN_SUCCESS, cover))),
        Err(err) => Ok(err.response()),
    }
}

// 推荐图片到首页
pub async fn recommand_photo_to_homepage(homepage_photo_request: web::Path<ModelRecommand::HomepagePhotoRequest>) -> Result<HttpResponse> {
    match ServiceRecommand::recommand_photo_to_homepage(homepage_photo_request.into_inner()).await {
        Ok(photo_res) => Ok(HttpResponse::Ok().json(ResponseBody::new(SUCCESS, MESSAGE_LOGIN_SUCCESS, photo_res))),
        Err(err) => Ok(err.response()),
    }
}
