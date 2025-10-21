use axum::body::Body;
use axum::http::Request;
use tower::ServiceExt;
use serde_json::Value;
use hyper::body::to_bytes;

use llm_inference::engine_mock::MockEngine;
use llm_inference::state::AppState;

pub async fn call_get_models() -> Value {
    let engine = MockEngine::new();
    let state = AppState::new(std::sync::Arc::new(engine));
    let app = llm_inference::routes::router().with_state(state);

    let req = Request::builder().method("GET").uri("/models").body(Body::empty()).unwrap();
    let resp = app.oneshot(req).await.unwrap();
    let bytes = to_bytes(resp.into_body()).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

pub async fn post_json_and_text(body: serde_json::Value) -> String {
    let engine = MockEngine::new();
    let state = AppState::new(std::sync::Arc::new(engine));
    let app = llm_inference::routes::router().with_state(state);

    let req = Request::builder()
        .method("POST")
        .uri("/chat/completions")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    let bytes = to_bytes(resp.into_body()).await.unwrap();
    String::from_utf8(bytes.to_vec()).unwrap()
}
