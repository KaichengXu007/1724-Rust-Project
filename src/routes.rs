use axum::{extract::State, routing::{get, post}, Router, response::IntoResponse, Json};
use axum::response::sse::{Sse, KeepAlive, Event};
use futures_util::StreamExt;
use crate::state::AppState;
use crate::models::{InferenceRequest, ModelsList};
use std::convert::Infallible;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/models", get(get_models))
        .route("/chat/completions", post(chat_completions))
}

async fn get_models(State(state): State<AppState>) -> impl IntoResponse {
    let list = state.engine.get_available_models().await;
    let resp = ModelsList { models: list };
    Json(resp)
}

async fn chat_completions(State(state): State<AppState>, Json(req): Json<InferenceRequest>) -> axum::response::Response {
    // call engine to get TokenStream
    if let Ok(stream) = state.engine.run_streaming_inference(req).await {
        // map TokenStream<Item = anyhow::Result<String>> to SSE Event stream
            let mapped = stream.map(|res| -> Result<Event, Infallible> {
                match res {
                    Ok(s) => Ok(Event::default().data(s)),
                    Err(e) => Ok(Event::default().data(format!("__ERROR__:{}", e))),
                }
            });

        // Convert mapped stream into axum::response::sse::Sse
        let keepalive = KeepAlive::new().interval(std::time::Duration::from_secs(15));
        let sse = Sse::new(mapped).keep_alive(keepalive);
        return sse.into_response();
    }

    // error branch
    let body = serde_json::json!({"error": "internal error"});
    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
}
