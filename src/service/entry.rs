use actix_web::{    
    http::{
        StatusCode,
    }
};
use crate::models::entry as ModelEntry;
use crate::service::error::ServiceError;
use crate::models::user_token::UserToken;
use crate::constants::message::{
    MESSAGE_INTERNAL_SERVER_ERROR,
};

pub async fn login(login_req: ModelEntry::LoginRequest) -> Result<ModelEntry::TokenBodyResponse, ServiceError> {
    match ModelEntry::login(login_req).await {
        Ok(logged_user) => {
            match serde_json::from_value(json!({ "token": UserToken::generate_token(&logged_user).await, "token_type": "api_key", "user": logged_user })) {
                Ok(res) => Ok(res),
                Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
            }
        },
        Err(err) => Err(err)
    }
}

pub async fn logout(logout_req: ModelEntry::LogoutRequest) -> Result<String, ServiceError> {
    match ModelEntry::logout(logout_req).await {
        Ok(res)  =>  Ok(res),
        Err(err) => Err(err)
    }
}