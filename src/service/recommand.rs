use actix_web::{    
    http::{
        StatusCode,
    }
};
use crate::models::recommand as ModelRecommand;
use crate::service::error::ServiceError;
use crate::constants::message::MESSAGE_INTERNAL_SERVER_ERROR;

pub async fn recommand_to_cover(cover_request: ModelRecommand::CoverRequest) -> Result<ModelRecommand::CoverResponse, ServiceError> {
    match ModelRecommand::recommand_to_cover(cover_request).await {
        Ok(res) => Ok(res),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
    }
}

pub async fn recommand_photo_to_homepage(homepage_photo_request: ModelRecommand::HomepagePhotoRequest) -> Result<ModelRecommand::HomepagePhotoResponse, ServiceError> {
    match ModelRecommand::recommand_photo_to_homepage(homepage_photo_request).await {
        Ok(res) => Ok(res),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
    }
}