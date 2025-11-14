use crate::domain::otp::Otp;
use crate::domain::user::UserEntity;
use crate::{api::ApiState, domain::email::Email};

use axum::extract::State;
use axum::{Json, extract};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: Email,
    pub otp: Otp,
}

pub async fn login(
    extract::State(state): State<ApiState>,
    extract::Json(payload): Json<LoginRequest>,
) -> &'static str {
    match UserEntity::get_by_email(&state, &payload.email).await {
        Ok(user) => {
            let otp = Otp::new(&user.id.to_string()).unwrap();

            if !otp.eq(&payload.otp) {
                "Invalid OTP"
            } else {
                "OK"
            }
        }
        Err(sqlx::Error::RowNotFound) => "Invalid OTP",
        Err(_) => "Internal Server Error",
    }
}
