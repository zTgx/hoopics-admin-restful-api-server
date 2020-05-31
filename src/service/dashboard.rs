use actix_web::http;
use crate::service::error::ServiceError;
use crate::constants::message::MESSAGE_DASHBOARD_GENERAL_FAILED;
use crate::models::dashboard as ModelDashboard;

pub async fn get_general_data() -> Result<ModelDashboard::GeneralData, ServiceError> {
    let mut user_count = None;
    if let Ok(res) = ModelDashboard::get_user_count().await {
        user_count = Some(res);
    }

    let mut photo_count = None;
    if let Ok(res) = ModelDashboard::get_photo_count().await {
        photo_count = Some(res);
    }

    let mut downloads = None;
    if let Ok(res) = ModelDashboard::get_photo_downloads().await {
        downloads = Some(res);
    }

    if user_count.is_none() && photo_count.is_none() && downloads.is_none() {
        return Err(ServiceError::new(http::StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_DASHBOARD_GENERAL_FAILED.to_string()));
    }

    Ok(
        ModelDashboard::GeneralData {
            user_count  : user_count,
            photo_count : photo_count,
            downloads   : downloads
        }
    )
}