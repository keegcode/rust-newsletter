use axum::extract::{Json, Path};
use serde::Deserialize;

use crate::domain::email::Email;

#[derive(Debug, Deserialize)]
pub struct SubscribeRequest {
    pub email: Email,
}

pub async fn subscribe(Path(topic): Path<String>, Json(payload): Json<SubscribeRequest>) {
    println!("Email {} subscribed to topic {}", payload.email, topic);
}
