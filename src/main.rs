use newsletter::{api::ApiState, config::CONFIG};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", CONFIG.port)).await?;

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&CONFIG.db.uri)
        .await?;

    let redis_url = format!("redis://:{}@localhost:6379", CONFIG.redis.password);
    let redis_client = redis::Client::open(redis_url)?;

    let state = ApiState {
        db,
        redis: redis_client,
    };

    println!("Starting server on port: {}", CONFIG.port);
    axum::serve(listener, newsletter::api::routes(&state).with_state(state)).await?;

    Ok(())
}
