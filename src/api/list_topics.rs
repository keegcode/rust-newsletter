use axum::{
    Json,
    extract::{Extension, State},
};
use serde::Deserialize;

use crate::{
    api::{ApiContext, ApiError, ApiState},
    domain::topic::Topic,
};

#[derive(Debug, Deserialize)]
pub struct CreateTopicRequest {
    pub name: String,
    pub template: String,
}

pub async fn list_topics(
    Extension(ctx): Extension<ApiContext>,
    State(state): State<ApiState>,
) -> Result<Json<Vec<Topic>>, ApiError> {
    Ok(Json(Topic::get_by_user_id(&state, ctx.user_id).await?))
}
