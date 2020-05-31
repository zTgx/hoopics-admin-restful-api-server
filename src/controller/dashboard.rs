use actix_web::{HttpResponse, Result};
use crate::service::dashboard as ServiceDashboard;
use crate::models::response::ResponseBody;
use crate::constants::message::{SUCCESS, MESSAGE_LOGIN_SUCCESS};

pub async fn get_general_data() -> Result<HttpResponse> {
    match ServiceDashboard::get_general_data().await {
        Ok(res) => Ok(HttpResponse::Ok().json(ResponseBody::new(SUCCESS, MESSAGE_LOGIN_SUCCESS, res))),
        Err(err) => Ok(err.response()),
    }
}