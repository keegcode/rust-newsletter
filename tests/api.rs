#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use newsletter::api::{self, ApiState};
    use newsletter::config::CONFIG;
    use serde_json::json;
    use sqlx::postgres::PgPoolOptions;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check() {
        let db = PgPoolOptions::new()
            .max_connections(5)
            .connect(&CONFIG.db.uri)
            .await
            .unwrap();

        let redis_url = format!("redis://:{}@localhost:6379", CONFIG.redis.password);
        let redis_client = redis::Client::open(redis_url).unwrap();

        let state = ApiState {
            db,
            redis: redis_client,
        };

        let app = api::routes(&state).with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/health")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), 2).await.unwrap();

        assert_eq!(body, "OK");
    }

    #[tokio::test]
    async fn test_subscribe() {
        let db = PgPoolOptions::new()
            .max_connections(5)
            .connect(&CONFIG.db.uri)
            .await
            .unwrap();

        let redis_url = format!("redis://:{}@localhost:6379", CONFIG.redis.password);
        let redis_client = redis::Client::open(redis_url).unwrap();

        let state = ApiState {
            db,
            redis: redis_client,
        };

        let app = api::routes(&state).with_state(state);

        let topic = "example";
        let body = json!({ "email": "example@gmail.com" }).to_string();

        let response = app
            .oneshot(
                Request::builder()
                    .uri(format!("/api/topics/{}/subscribe", topic))
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
