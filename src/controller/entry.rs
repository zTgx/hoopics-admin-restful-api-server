use actix_web::{web, HttpResponse, Result};
use crate::service::entry as ServiceEntry;
use crate::models::entry as ModelEntry;
use crate::models::response::ResponseBody;
use crate::constants::message::{SUCCESS, MESSAGE_LOGIN_SUCCESS, MESSAGE_LOGOUT_SUCCESS};

pub async fn login(login_req: web::Json<ModelEntry::LoginRequest>) -> Result<HttpResponse> {
    match ServiceEntry::login(login_req.into_inner()).await { 
        Ok(res) => Ok(HttpResponse::Ok().json(ResponseBody::new(SUCCESS, MESSAGE_LOGIN_SUCCESS, res))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn logout(logout_req: web::Json<ModelEntry::LogoutRequest>) -> Result<HttpResponse> {
    match ServiceEntry::logout(logout_req.into_inner()).await {
        Ok(res) => Ok(HttpResponse::Ok().json(ResponseBody::new(SUCCESS, MESSAGE_LOGOUT_SUCCESS, res))),
        Err(err) => Ok(err.response()), 
    }
}
