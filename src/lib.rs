// Crate root: re-exports internal modules that implement parsing, model inference logic,
// the adapter trait for inference engines, application state, API models, and routes.
//
// Recent changes (commit):
// - Added unit tests and integration tests with a MockEngine to verify routing and SSE
// - Added helper test utilities under tests/ for consistent request construction
// - Added configuration system with TOML support
// - Added API key authentication and rate limiting middleware
pub mod engine;
pub mod state;
pub mod models;
pub mod routes;
pub mod engine_mock;
pub mod config;
pub mod middleware;

#[cfg(test)]
mod tests {
use super::*;
use axum::body::Body;
use axum::http::Request;
use tower::ServiceExt;

#[tokio::test]
async fn lib_router_smoke() {
// test router responds to /models
use metrics_exporter_prometheus::PrometheusBuilder;
let builder = PrometheusBuilder::new();
// Use build_recorder() to get the recorder and handle without installing global state
let recorder = builder.build_recorder();
let handle = recorder.handle();

let config = config::Config::default();
let state = state::AppState::new(std::sync::Arc::new(engine_mock::MockEngine::new()), handle, config);
let app = routes::router().with_state(state);
let req = Request::builder().method("GET").uri("/models").body(Body::empty()).unwrap();
let resp = app.oneshot(req).await.expect("request");
assert!(resp.status().is_success());
}

#[tokio::test]
async fn test_persistence_flow() {
    // Clean up previous test file if exists
    let _ = std::fs::remove_file("sessions.json");

    use metrics_exporter_prometheus::PrometheusBuilder;
    
    let builder = PrometheusBuilder::new();
    let recorder = builder.build_recorder();
    let handle = recorder.handle();

    let engine = std::sync::Arc::new(engine_mock::MockEngine::new());
    let config = config::Config::default();
    let state = state::AppState::new(engine.clone(), handle.clone(), config.clone());
    
    // Manually insert a session to simulate a chat
    {
        let mut sessions = state.sessions.lock().await;
        sessions.insert("test-session".to_string(), vec![
            models::ChatMessage { role: "user".to_string(), content: "hello".to_string() }
        ]);
    }
    
    // Trigger save
    state.save_sessions().await;
    
    // Verify file exists
    assert!(std::path::Path::new("sessions.json").exists());
    
    // Create new state and verify load
    let state2 = state::AppState::new(engine, handle, config);
    let sessions = state2.sessions.lock().await;
    assert!(sessions.contains_key("test-session"));
    assert_eq!(sessions.get("test-session").unwrap()[0].content, "hello");
    
    // Cleanup
    let _ = std::fs::remove_file("sessions.json");
}
}
