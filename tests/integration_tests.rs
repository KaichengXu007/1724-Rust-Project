use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use llm_inference::{config::Config, engine_mock::MockEngine, models::*, routes, state::AppState};
use metrics_exporter_prometheus::PrometheusBuilder;
use serde_json::json;
use std::sync::Arc;
use tower::ServiceExt;

async fn setup_test_state() -> AppState {
    let builder = PrometheusBuilder::new();
    let recorder = builder.build_recorder();
    let handle = recorder.handle();
    let engine = Arc::new(MockEngine::new());
    let config = Config::default();
    AppState::new(engine, handle, config).await.unwrap()
}

#[tokio::test]
async fn test_health_endpoint() {
    let state = setup_test_state().await;
    let app = routes::router().with_state(state);

    let req = Request::builder()
        .method("GET")
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_readiness_endpoint() {
    let state = setup_test_state().await;
    let app = routes::router().with_state(state);

    let req = Request::builder()
        .method("GET")
        .uri("/readiness")
        .body(Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_models_list() {
    let state = setup_test_state().await;
    let app = routes::router().with_state(state);

    let req = Request::builder()
        .method("GET")
        .uri("/models")
        .body(Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
    let models: ModelsList = serde_json::from_slice(&body).unwrap();
    assert!(!models.models.is_empty());
}

#[tokio::test]
async fn test_completions_endpoint() {
    let state = setup_test_state().await;
    let app = routes::router().with_state(state);

    let payload = json!({
        "model": "mock-model",
        "prompt": "Hello",
        "max_tokens": 50,
        "stream": false
    });

    let req = Request::builder()
        .method("POST")
        .uri("/completions")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload).unwrap()))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_chat_completions_endpoint() {
    let state = setup_test_state().await;
    let app = routes::router().with_state(state);

    let payload = json!({
        "model-name": "mock-model",
        "prompt": "Hello",
        "max-token": 50,
        "device": "cpu"
    });

    let req = Request::builder()
        .method("POST")
        .uri("/chat/completions")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload).unwrap()))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_session_management() {
    let state = setup_test_state().await;
    let app = routes::router().with_state(state.clone());

    // List sessions
    let req = Request::builder()
        .method("GET")
        .uri("/sessions")
        .body(Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_prompt_length_validation() {
    let mut config = Config::default();
    config.limits.max_prompt_length = 10;

    let builder = PrometheusBuilder::new();
    let recorder = builder.build_recorder();
    let handle = recorder.handle();
    let engine = Arc::new(MockEngine::new());
    let state = AppState::new(engine, handle, config).await.unwrap();
    let app = routes::router().with_state(state);

    let payload = json!({
        "model": "mock-model",
        "prompt": "This is a very long prompt that exceeds the limit",
        "max_tokens": 50,
        "stream": false
    });

    let req = Request::builder()
        .method("POST")
        .uri("/completions")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload).unwrap()))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_metrics_endpoint() {
    let state = setup_test_state().await;
    let app = routes::router().with_state(state);

    let req = Request::builder()
        .method("GET")
        .uri("/metrics")
        .body(Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}
