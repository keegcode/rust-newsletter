use axum::extract::State;
use axum::{Json, extract};
use serde::Deserialize;

use crate::domain::{email::Email, otp::Otp, user::UserEntity};

use super::ApiState;

#[derive(Debug, Deserialize)]
pub struct RequestOtpRequest {
    pub email: Email,
}

pub async fn request_otp(
    extract::State(state): State<ApiState>,
    extract::Json(payload): Json<RequestOtpRequest>,
) -> &'static str {
    match UserEntity::get_by_email(&state, &payload.email).await {
        Ok(user) => {
            let otp = Otp::new(&user.id.to_string());

            println!("{:?}", payload);
            println!("{:?}", otp);

            "OK"
        }
        Err(sqlx::Error::RowNotFound) => {
            let mut user = UserEntity::new(&payload.email);
            user.save(&state).await.unwrap();

            let otp = Otp::new(&user.id.to_string());

            println!("{:?}", payload);
            println!("{:?}", otp);

            "OK"
        }
        Err(_) => "Internal Server Error",
    }
}
