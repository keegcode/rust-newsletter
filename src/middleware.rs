use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::IntoResponse,
};

use crate::{
    api::{ApiContext, ApiError, ApiState},
    auth,
};

pub async fn require_auth(
    State(state): State<ApiState>,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, ApiError> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or(ApiError::Forbidden)?;

    let auth_header_str = auth_header
        .to_str()
        .map_err(|_| ApiError::Forbidden)?
        .to_string();

    let token = auth_header_str
        .strip_prefix("Bearer ")
        .ok_or(ApiError::Forbidden)?;

    let user_id = auth::get_user_id(&state, token).await?;

    let context = ApiContext { user_id };
    request.extensions_mut().insert(context);
    Ok(next.run(request).await)
}
