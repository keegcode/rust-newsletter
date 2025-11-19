use crate::api::ApiError;
use crate::auth::{self, get_token};
use crate::domain::otp::Otp;
use crate::domain::user::UserEntity;
use crate::{api::ApiState, domain::email::Email};

use axum::extract::State;
use axum::{Json, extract};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: Email,
    pub otp: Otp,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn login(
    extract::State(state): State<ApiState>,
    extract::Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    let user = UserEntity::get_by_email(&state, &payload.email)
        .await?
        .ok_or(ApiError::BadRequest("Invalid OTP".into()))?;

    let otp = Otp::new(&user.id.to_string())?;

    if !otp.eq(&payload.otp) {
        return Err(ApiError::BadRequest("Invalid OTP".to_string()));
    }

    let token = get_token();
    auth::set_user_id(&state, &token, user.id).await?;
    Ok(Json(LoginResponse { token }))
}
