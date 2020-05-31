use actix_web::{    
    http::{
        StatusCode,
    },
};
use crate::service::error::ServiceError;
use crate::constants::message::{
    MESSAGE_INTERNAL_SERVER_ERROR,
};

use crate::models::photo;

pub async fn get_unapproved_photos(start: u32, step: u32) -> Result<photo::PhotoResponse, ServiceError> {
    match photo::get_photos_by_approve_status(photo::ApproveStatus::Unapproved, start, step).await {
        Ok(res) => Ok(res),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
    }
}

pub async fn get_approved_photos(start: u32, step: u32) -> Result<photo::PhotoResponse, ServiceError> {
    match photo::get_photos_by_approve_status(photo::ApproveStatus::Approved, start, step).await {
        Ok(res) => Ok(res),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
    }
}

pub async fn get_reject_photos(start: u32, step: u32) -> Result<photo::PhotoResponse, ServiceError> {
    match photo::get_photos_by_approve_status(photo::ApproveStatus::Reject, start, step).await {
        Ok(res) => Ok(res),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
    }
}

pub async fn get_all_photos(start: u32, step: u32) -> Result<photo::PhotoResponse, ServiceError> {
    match photo::get_all_photos(start, step).await {
        Ok(res) => Ok(res),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
    }
}

pub async fn update_approve_status(photo_id: i32, status: i32) -> Result<photo::ApproveStatusResponse, ServiceError> {
    let approve_status = photo::ApproveStatus::from(status);
    let info = photo::ApproveStatusRequest::new(photo_id, approve_status);
    match photo::update_approve_status(info).await {
        Ok(res) => Ok(res),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
    }
}

pub async fn delete_photo(photo_id: i32) -> Result<photo::DeletePhotoResponse, ServiceError> {
    match photo::delete_photo(photo_id).await {
        Ok(res) => Ok(res),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
    }
}

