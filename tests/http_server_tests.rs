use axum::body::Body;
use axum::http::StatusCode;
use markdown_live_preview::SharedState;
use markdown_live_preview::http_server::build_router;
use tower::ServiceExt;

#[tokio::test]
async fn test_http_server_serves_preview() {
    let state = SharedState::default();
    let app = build_router(state);

    let response = app
        .oneshot(
            axum::http::Request::builder()
                .uri("/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    // Further checks can be made on the response body to ensure HTML content is served
}
