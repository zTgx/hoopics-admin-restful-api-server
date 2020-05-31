use serde::{ Deserialize, Serialize };
use diesel::prelude::*;
use actix_web::{    
    http::{
        StatusCode,
    },
};
use bcrypt::verify;

use crate::service::error::ServiceError;
use crate::constants::message::MESSAGE_LOGIN_FAILED;

use crate::config::db::ADMINDBPOOL;
pub use crate::schema::entry::Admin;
use crate::schema::entry::admin::dsl::*;
use crate::models::user_token::UserToken;

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginRequest { 
    pub email   : String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdminInfo {
    pub user_id : String,
    pub name    : String,
    pub avatar  : String,
    pub email   : String,
    pub group   : i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenBodyResponse {
    pub token       : String,
    pub token_type  : String,
    pub user        : AdminInfo,
}

pub async fn login(login_req: LoginRequest) -> Result<AdminInfo, ServiceError> {
    let conn = &ADMINDBPOOL.get().unwrap();

    if let Ok(user_to_verify) = admin.filter(email.eq(&login_req.email)).get_result::<Admin>(conn) {   
        if !user_to_verify.password.is_empty() &&
            verify(&login_req.password, &user_to_verify.password).unwrap() {
                let admin_info = AdminInfo {
                    user_id : user_to_verify.user_id.clone(),
                    name    : user_to_verify.name.clone(),
                    avatar  : user_to_verify.avatar.clone(),
                    email   : user_to_verify.email.clone(),
                    group   : user_to_verify.group,
                };

            return Ok(admin_info);
        }
    }

    Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_LOGIN_FAILED.to_string()))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogoutRequest {
    pub token       : String,
    pub token_type  : String,
}
pub async fn logout(logout_req: LogoutRequest) -> Result<String, ServiceError> {
    // UserToken::disable_token(&logout_req.token);
    // Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_LOGIN_FAILED.to_string()))

    Ok("token cleared".to_string())
}
