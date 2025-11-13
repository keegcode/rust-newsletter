use axum::Json;
use serde::Deserialize;

use crate::domain::{email::Email, otp::Otp};

#[derive(Debug, Deserialize)]
pub struct RequestOtpRequest {
    pub email: Email,
}

pub async fn request_otp(Json(payload): Json<RequestOtpRequest>) {
    println!("{:?}", payload);
    println!("{:?}", Otp::new());
}
