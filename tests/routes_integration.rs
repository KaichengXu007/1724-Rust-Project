// Integration tests for the HTTP routes using a MockEngine.
// Tests verify:
// - GET /models returns a JSON list of available models
// - POST /chat/completions returns SSE-formatted token stream when using MockEngine
//
// Recent changes: added MockEngine-based tests and hyper::body::to_bytes for stable body handling.
use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt; // for .oneshot()
use serde_json::json;
use hyper::body::to_bytes;

#[tokio::test]
async fn test_get_models() {
    let engine = llm_inference::engine_mock::MockEngine::new();
    let state = llm_inference::state::AppState::new(std::sync::Arc::new(engine));
    let app = llm_inference::routes::router().with_state(state);

    let req = Request::builder()
        .method("GET")
        .uri("/models")
        .body(Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let bytes = to_bytes(resp.into_body()).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert!(v["models"].is_array());
    assert_eq!(v["models"][0], "mock-model");
}

#[tokio::test]
async fn test_chat_completions_sse() {
    let engine = llm_inference::engine_mock::MockEngine::new();
    let state = llm_inference::state::AppState::new(std::sync::Arc::new(engine));
    let app = llm_inference::routes::router().with_state(state);

    let body = json!({
        "model-name": "mock-model",
        "prompt": "world",
        "max-token": 10,
        "temperature": 0.7,
        "top-p": 0.95,
        "top-k": 10,
        "repeat-penalty": 1.0,
        "stop": [],
        "device": "cpu"
    });

    let req = Request::builder()
        .method("POST")
        .uri("/chat/completions")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let bytes = to_bytes(resp.into_body()).await.unwrap();
    let s = String::from_utf8(bytes.to_vec()).unwrap();

    // SSE events are formatted as 'data: ...\n\n'
    assert!(s.contains("hello"));
    assert!(s.contains("world"));
}
