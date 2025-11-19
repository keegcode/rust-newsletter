use axum::{
    Json,
    extract::{Extension, State},
};
use serde::Deserialize;

use crate::{
    api::{ApiContext, ApiError, ApiState},
    domain::topic::{NewTopicPayload, Topic},
};

#[derive(Debug, Deserialize)]
pub struct CreateTopicRequest {
    pub name: String,
    pub template: String,
}

pub async fn create_topic(
    Extension(ctx): Extension<ApiContext>,
    State(state): State<ApiState>,
    Json(body): Json<CreateTopicRequest>,
) -> Result<String, ApiError> {
    let mut topic = Topic::new(&NewTopicPayload {
        user_id: ctx.user_id,
        name: body.name,
        template: body.template,
    });
    topic.save(&state).await?;

    Ok(topic.id.to_string())
}
