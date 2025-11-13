pub mod health_check;
pub mod login;
pub mod request_otp;
pub mod subscribe;

use health_check::health_check;
use login::login;
use request_otp::request_otp;
use sqlx::PgPool;
use subscribe::subscribe;

use axum::{Router, routing::get, routing::post};

#[derive(Clone)]
pub struct ApiState {
    pub db: PgPool,
}

pub fn routes() -> Router<ApiState> {
    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/auth/login", post(login))
        .route("/api/auth/request-otp", post(request_otp))
        //.route("/api/topics", post(create_topic))
        //.route("/api/topics", get(list_topics))
        //.route("/api/topics/{topic}", put(update_topic))
        //.route("/api/topics/{topic}", delete(delete_topic))
        .route("/api/topics/{topic}/subscribe", post(subscribe))
    //.route("/api/templates", post(create_template))
    //.route("/api/templates", get(list_templates))
    //.route("/api/templates/{template}", delete(delete_template))
    //.route("/api/templates/{template}", put(update_template))
}
