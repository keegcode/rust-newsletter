#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use newsletter::api;
    use serde_json::json;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check() {
        let app = api::routes();

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
        let app = api::routes();

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
