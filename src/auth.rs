use rand::{RngCore, SeedableRng, rngs::StdRng};
use redis::AsyncCommands;

use crate::api::{ApiError, ApiState};

pub async fn get_user_id(state: &ApiState, token: &str) -> Result<i64, ApiError> {
    let mut conn = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let id: String = conn.get(token).await?;
    Ok(id.parse()?)
}

pub async fn set_user_id(state: &ApiState, token: &str, user_id: i64) -> Result<(), ApiError> {
    let mut conn = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    Ok(conn.set_ex(token, user_id.to_string(), 3600).await?)
}

pub fn get_token() -> String {
    let mut rng = StdRng::from_os_rng();
    let mut buf = vec![0u8; 32];
    rng.fill_bytes(&mut buf);
    hex::encode(buf)
}
