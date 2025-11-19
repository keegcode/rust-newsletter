pub mod create_topic;
pub mod health_check;
pub mod list_topics;
pub mod login;
pub mod request_otp;
pub mod subscribe;

use std::num::ParseIntError;

use create_topic::create_topic;
use health_check::health_check;
use list_topics::list_topics;
use login::login;
use redis::RedisError;
use request_otp::request_otp;
use sqlx::PgPool;
use subscribe::subscribe;

use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};

use crate::middleware;

#[derive(Debug, Clone)]
pub struct ApiState {
    pub db: PgPool,
    pub redis: redis::Client,
}

#[derive(Debug)]
pub enum ApiError {
    Sqlx(sqlx::Error),
    Redis(RedisError),
    NotFound,
    BadRequest(String),
    Internal(String),
    Forbidden,
}

#[derive(Debug, Clone)]
pub struct ApiContext {
    pub user_id: i64,
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    error: String,
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => ApiError::NotFound,
            e => ApiError::Sqlx(e),
        }
    }
}

impl From<redis::RedisError> for ApiError {
    fn from(err: redis::RedisError) -> Self {
        ApiError::Redis(err)
    }
}

impl From<ParseIntError> for ApiError {
    fn from(err: ParseIntError) -> Self {
        ApiError::Internal(err.to_string())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message): (StatusCode, String) = match self {
            ApiError::Sqlx(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Interal Server Error".to_string(),
            ),
            ApiError::Redis(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Interal Server Error".to_string(),
            ),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Not Found".to_string()),
            ApiError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Interal Server Error".to_string(),
            ),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),
        };

        let response = Json(ErrorResponse { error: message });

        (status, response).into_response()
    }
}

pub fn routes(state: &ApiState) -> Router<ApiState> {
    let public = Router::new()
        .nest(
            "/auth",
            Router::new()
                .route("/login", post(login))
                .route("/request-otp", post(request_otp)),
        )
        .route("/health", get(health_check));

    let protected = Router::new()
        .nest(
            "/topics",
            Router::new()
                .route("/", post(create_topic))
                .route("/", get(list_topics))
                .route("/{topic}/subscribe", post(subscribe)),
        )
        .nest("/templates", Router::new())
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::require_auth,
        ));

    let api = Router::new().merge(public).merge(protected);

    Router::new().nest("/api", api)
}
