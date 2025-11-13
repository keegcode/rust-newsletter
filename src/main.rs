use newsletter::{api::ApiState, config::CONFIG};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", CONFIG.port)).await?;

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&CONFIG.db.uri)
        .await?;

    let state = ApiState { db };

    println!("Starting server on port: {}", CONFIG.port);
    axum::serve(listener, newsletter::api::routes().with_state(state)).await?;

    Ok(())
}
