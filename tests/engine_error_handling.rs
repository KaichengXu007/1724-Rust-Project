use axum::body::Body;
use axum::http::Request;
use tower::ServiceExt;
use hyper::body::to_bytes;
use serde_json::json;

use llm_inference::engine::InferenceEngine;
use llm_inference::state::AppState;
use std::sync::Arc;
use async_trait::async_trait;
use anyhow::Result as AnyResult;

// A mock engine that returns an error
struct ErrorEngine;

#[async_trait]
impl InferenceEngine for ErrorEngine {
    async fn get_available_models(&self) -> Vec<String> { vec![] }
    async fn run_streaming_inference(&self, _request: llm_inference::models::InferenceRequest) -> AnyResult<llm_inference::engine::TokenStream> {
        Err(anyhow::anyhow!("engine failure"))
    }
}

#[tokio::test]
async fn engine_error_produces_500() {
    let engine: Arc<dyn InferenceEngine> = Arc::new(ErrorEngine);
    let state = AppState::new(engine);
    let app = llm_inference::routes::router().with_state(state);

    let body = json!({"model-name":"mock-model","model-dir":"models/","prompt":"x","repeat-penalty":1.0,"stop":[],"device":"cpu"});
    let req = Request::builder().method("POST").uri("/chat/completions").header("content-type","application/json").body(Body::from(body.to_string())).unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 500);
    let bytes = to_bytes(resp.into_body()).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert!(v.get("error").is_some());
}
