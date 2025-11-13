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
    extract::Json(payload): Json<LoginRequest>,
    extract::State(state): State<ApiState>,
) {
    let user = UserEntity::get_by_email(&state, &payload.email)
        .await
        .unwrap();

    let otp = Otp::new(payload.email.into());

    if otp != payload.otp {}
}
