use std::sync::Arc;
use axum::{routing::{get, post}, Json, Router};
use axum::extract::Extension;
use axum::response::sse::{Sse, Event};
use axum::response::{IntoResponse, Response};
use tokio::signal;
use futures_util::{TryStreamExt};

use llm_inference::engine::InferenceEngine;
use llm_inference::engine_mock::MockEngine;
use llm_inference::models::InferenceRequest;
use llm_inference::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let engine: Arc<dyn InferenceEngine> = Arc::new(MockEngine::new());
    let app_state = AppState::new(engine);

    async fn models_handler(Extension(state): Extension<AppState>) -> impl IntoResponse {
        let models = state.engine.get_available_models().await;
        Json(serde_json::json!({"models": models}))
    }

    async fn chat_handler(Extension(state): Extension<AppState>, Json(req): Json<InferenceRequest>) -> Response {
        match state.engine.run_streaming_inference(req).await {
            Ok(stream) => {
                let mapped = stream.map_ok(|s| Event::default().data(s));
                Sse::new(mapped).into_response()
            }
            Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("{}", e)}))).into_response(),
        }
    }

    let app = Router::new()
        .route("/models", get(models_handler))
        .route("/chat/completions", post(chat_handler))
        .layer(axum::Extension(app_state));

    let addr = std::net::SocketAddr::from(([127,0,0,1], 3000));
    tracing::info!("mock server listening on {}", addr);

    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let _ = signal::ctrl_c().await;
}
