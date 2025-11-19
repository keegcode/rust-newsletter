use axum::extract::State;
use axum::{Json, extract};
use serde::Deserialize;

use crate::api::ApiError;
use crate::domain::{email::Email, otp::Otp, user::UserEntity};

use super::ApiState;

#[derive(Debug, Deserialize)]
pub struct RequestOtpRequest {
    pub email: Email,
}

pub async fn request_otp(
    extract::State(state): State<ApiState>,
    extract::Json(payload): Json<RequestOtpRequest>,
) -> Result<(), ApiError> {
    let user = UserEntity::get_by_email(&state, &payload.email).await?;

    match user {
        Some(user) => {
            let otp = Otp::new(&user.id.to_string())?;
            println!("{:?}", otp);

            Ok(())
        }
        None => {
            let mut user = UserEntity::new(&payload.email);
            user.save(&state).await?;

            let otp = Otp::new(&user.id.to_string())?;
            println!("{:?}", otp);

            Ok(())
        }
    }
}
