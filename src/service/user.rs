use actix_web::{    
    http::{
        StatusCode,
    },
};
use crate::service::error::ServiceError;
use crate::constants::message::{
    MESSAGE_INTERNAL_SERVER_ERROR,
};
use crate::models::user as ModelUser;

pub async fn get_user_by_id(user_id: String) -> Result<ModelUser::UserInfo, ServiceError> {
    match ModelUser::get_user_by_id(user_id).await {
        Ok(res) => Ok(res),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
    } 
}

pub async fn get_all_users(pagination: ModelUser::Pagination) -> Result<ModelUser::UserResponse, ServiceError> {
    match ModelUser::get_all_users(pagination).await {
        Ok(res) => Ok(res),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
    }
}

pub async fn add_user_to_blacklist(mute_request: ModelUser::MuteRequest) -> Result<ModelUser::MuteResponse, ServiceError> {
    match ModelUser::add_user_to_blacklist(mute_request) {
        Ok(res) => Ok(res),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
    }
}

pub async fn remove_user_from_blacklist(mute_request: ModelUser::MuteRequest) -> Result<ModelUser::MuteResponse, ServiceError> {
    match ModelUser::remove_user_from_blacklist(mute_request) {
        Ok(res) => Ok(res),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
    }
}

pub async fn delete_user(delete_user_request: ModelUser::DeleteUserRequest) -> Result<ModelUser::DeleteUserResponse, ServiceError> {
    match ModelUser::delete_user(delete_user_request) {
        Ok(res) => Ok(res),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
    }
}
