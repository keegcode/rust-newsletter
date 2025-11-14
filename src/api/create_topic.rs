use axum::extract;
use axum::extract::State;

use crate::domain::topic::{NewTopicPayload, Topic};

use super::ApiState;

pub async fn create_topic(extract::State(state): State<ApiState>) -> &'static str {
    let topic = Topic::new(&NewTopicPayload {
        user_id,
        name,
        template,
    });
    topic.save()
}
