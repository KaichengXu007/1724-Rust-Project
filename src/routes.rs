use crate::models::{ChatMessage, CompletionRequest, InferenceRequest, ModelsList};
use crate::state::AppState;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use futures_util::StreamExt;
use metrics::{counter, histogram, increment_counter};
use std::convert::Infallible;
use std::time::Instant;

const MAX_HISTORY_LENGTH: usize = 20; // Keep last 20 messages (approx 10 rounds)

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/models", get(get_models))
        .route("/models/:model_id", get(get_model_info))
        .route("/sessions", get(list_sessions))
        .route("/completions", post(completions))
        .route("/chat/completions", post(chat_completions))
        .route("/chat/ws", get(chat_ws))
        .route(
            "/chat/history/:session_id",
            get(get_history).delete(delete_session),
        )
        .route("/chat/history/:session_id/rollback", post(rollback_history))
        .route("/health", get(health_check))
        .route("/readiness", get(readiness_check))
        .route("/metrics", get(metrics_handler))
}

async fn health_check() -> impl IntoResponse {
    increment_counter!("health_check_requests_total");
    Json(serde_json::json!({
        "status": "ok",
        "uptime": "running",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn readiness_check(State(state): State<AppState>) -> impl IntoResponse {
    increment_counter!("readiness_check_requests_total");

    // Check if engine is ready
    let models = state.engine.get_available_models().await;
    let ready = !models.is_empty();

    if ready {
        Json(serde_json::json!({
            "status": "ready",
            "models_available": models.len(),
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    } else {
        Json(serde_json::json!({
            "status": "not_ready",
            "reason": "No models available",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }
}

async fn metrics_handler(State(state): State<AppState>) -> String {
    state.metrics_handle.render()
}

// Helper to prune history
fn prune_history(history: &mut Vec<ChatMessage>) {
    if history.len() > MAX_HISTORY_LENGTH {
        // Always keep the system prompt if it exists at index 0
        let has_system = history.first().map(|m| m.role == "system").unwrap_or(false);

        if has_system {
            let system_msg = history.remove(0);
            let remove_count = history.len().saturating_sub(MAX_HISTORY_LENGTH - 1);
            if remove_count > 0 {
                history.drain(0..remove_count);
            }
            history.insert(0, system_msg);
        } else {
            let remove_count = history.len().saturating_sub(MAX_HISTORY_LENGTH);
            if remove_count > 0 {
                history.drain(0..remove_count);
            }
        }
    }
}

async fn get_models(State(state): State<AppState>) -> impl IntoResponse {
    increment_counter!("models_list_requests_total");
    let list = state.engine.get_available_models().await;
    let resp = ModelsList { models: list };
    Json(resp)
}

async fn get_model_info(
    State(state): State<AppState>,
    Path(model_id): Path<String>,
) -> impl IntoResponse {
    increment_counter!("model_info_requests_total");

    // Find model config
    let model_config = state
        .config
        .models
        .available_models
        .iter()
        .find(|m| m.id == model_id || m.name == model_id);

    if let Some(config) = model_config {
        Json(serde_json::json!({
            "id": config.id,
            "name": config.name,
            "context_length": config.context_length,
            "quantization": config.quantization,
        }))
    } else {
        Json(serde_json::json!({
            "error": "Model not found"
        }))
    }
}

async fn list_sessions(State(state): State<AppState>) -> impl IntoResponse {
    let sessions = state.sessions.lock().await;
    let keys: Vec<String> = sessions.keys().cloned().collect();
    Json(keys)
}

async fn delete_session(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
) -> impl IntoResponse {
    {
        let mut sessions = state.sessions.lock().await;
        sessions.remove(&session_id);
    }
    state.delete_session_record(&session_id).await;
    axum::http::StatusCode::NO_CONTENT
}

async fn rollback_history(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let amount = payload.get("amount").and_then(|v| v.as_u64()).unwrap_or(1) as usize;

    {
        let mut sessions = state.sessions.lock().await;

        if let Some(history) = sessions.get_mut(&session_id) {
            let len = history.len();
            if len > amount {
                history.truncate(len - amount);
            } else {
                // Don't remove system prompt if possible, or just clear all except system
                let has_system = history.first().map(|m| m.role == "system").unwrap_or(false);
                if has_system {
                    history.truncate(1);
                } else {
                    history.clear();
                }
            }
        }
    }
    state.persist_session(&session_id).await;
    Json(serde_json::json!({"status": "ok"}))
}

async fn get_history(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
) -> impl IntoResponse {
    increment_counter!("history_requests_total");
    let sessions = state.sessions.lock().await;
    let history = sessions.get(&session_id).cloned().unwrap_or_default();
    Json(history)
}

async fn completions(
    State(state): State<AppState>,
    Json(req): Json<CompletionRequest>,
) -> axum::response::Response {
    increment_counter!("completions_requests_total");
    let start_time = Instant::now();

    // Validate prompt length
    if let Err(e) = state.validate_prompt_length(&req.prompt) {
        return (
            axum::http::StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": e.to_string()
            })),
        )
            .into_response();
    }

    // Clamp max_tokens to config limit
    let max_tokens = req.max_tokens.min(state.config.limits.max_response_tokens);

    // Convert to InferenceRequest
    let inference_req = InferenceRequest {
        model_name: req.model.clone(),
        model_dir: None,
        prompt: req.prompt.clone(),
        messages: None,
        session_id: None,
        max_token: max_tokens,
        temperature: req.temperature,
        top_p: req.top_p,
        top_k: 10,
        repeat_penalty: 1.0,
        stop: req.stop.clone(),
        device: state.config.models.default_device.clone(),
    };

    match state.run_inference_guarded(inference_req).await {
        Ok(mut stream) => {
            if req.stream {
                // Return SSE stream
                let wrapped_stream = async_stream::stream! {
                    let mut token_count = 0;
                    let _stream_start = Instant::now();

                    while let Some(result) = stream.next().await {
                        match result {
                            Ok(token) => {
                                token_count += 1;
                                yield Ok::<Event, Infallible>(Event::default().data(token));
                            }
                            Err(e) => {
                                tracing::error!("Stream error: {:?}", e);
                                yield Ok::<Event, Infallible>(Event::default().data(format!("__ERROR__:{}", e)));
                            }
                        }
                    }

                    let duration = start_time.elapsed().as_secs_f64();
                    histogram!("completions_duration_seconds", duration);
                    counter!("completions_tokens_total", token_count);

                    // Calculate tokens per second
                    if duration > 0.0 {
                        let tokens_per_second = token_count as f64 / duration;
                        histogram!("completions_tokens_per_second", tokens_per_second);
                    }
                };

                let keepalive = KeepAlive::new().interval(std::time::Duration::from_secs(15));
                let sse = Sse::new(wrapped_stream).keep_alive(keepalive);
                sse.into_response()
            } else {
                // Collect full response
                let mut full_response = String::new();
                let mut token_count = 0;

                while let Some(result) = stream.next().await {
                    match result {
                        Ok(token) => {
                            token_count += 1;
                            full_response.push_str(&token);
                        }
                        Err(e) => {
                            return (
                                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                Json(serde_json::json!({
                                    "error": e.to_string()
                                })),
                            )
                                .into_response();
                        }
                    }
                }

                let duration = start_time.elapsed().as_secs_f64();
                histogram!("completions_duration_seconds", duration);
                counter!("completions_tokens_total", token_count);

                if duration > 0.0 {
                    let tokens_per_second = token_count as f64 / duration;
                    histogram!("completions_tokens_per_second", tokens_per_second);
                }

                Json(serde_json::json!({
                    "text": full_response,
                    "model": req.model,
                    "tokens": token_count,
                    "duration_seconds": duration,
                    "tokens_per_second": if duration > 0.0 { Some(token_count as f64 / duration) } else { None }
                })).into_response()
            }
        }
        Err(e) => {
            tracing::error!("Inference error: {:?}", e);
            increment_counter!("completions_errors_total");
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": e.to_string()
                })),
            )
                .into_response()
        }
    }
}

async fn chat_completions(
    State(state): State<AppState>,
    Json(mut req): Json<InferenceRequest>,
) -> axum::response::Response {
    increment_counter!("chat_completions_requests_total");
    let start_time = Instant::now();

    // Validate prompt length
    if let Err(e) = state.validate_prompt_length(&req.prompt) {
        return (
            axum::http::StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": e.to_string()
            })),
        )
            .into_response();
    }

    // Clamp max_token to config limit
    req.max_token = req.max_token.min(state.config.limits.max_response_tokens);

    // Handle Session: if session_id is present, append prompt to history and use history as context
    let session_id = req.session_id.clone();
    if let Some(sid) = &session_id {
        // Check session limit
        if let Err(e) = state.check_session_limit().await {
            return (
                axum::http::StatusCode::TOO_MANY_REQUESTS,
                Json(serde_json::json!({
                    "error": e.to_string()
                })),
            )
                .into_response();
        }

        let mut sessions = state.sessions.lock().await;
        let history = sessions.entry(sid.clone()).or_insert_with(|| {
            vec![ChatMessage {
                role: "system".to_string(),
                content: "You are a helpful AI assistant.".to_string(),
            }]
        });

        // Append current user prompt
        history.push(ChatMessage {
            role: "user".to_string(),
            content: req.prompt.clone(),
        });

        // Prune history if too long
        prune_history(history);

        // Use full history for inference
        req.messages = Some(history.clone());
    }
    if let Some(sid) = session_id.as_ref() {
        state.persist_session(sid).await;
    }

    // call engine to get TokenStream
    match state.run_inference_guarded(req).await {
        Ok(mut stream) => {
            let sessions = state.sessions.clone();
            let sid_clone = session_id.clone();
            let state_clone = state.clone();

            // Wrap the stream to capture the full response
            let wrapped_stream = async_stream::stream! {
                let mut full_response = String::new();
                let mut token_count = 0;
                let _stream_start = Instant::now();
                let mut session_cancelled = false;

                while let Some(result) = stream.next().await {
                    match result {
                        Ok(token) => {
                            if let Some(ref sid) = sid_clone {
                                let session_still_exists = {
                                    let guard = sessions.lock().await;
                                    guard.contains_key(sid)
                                };
                                if !session_still_exists {
                                    tracing::info!("Session {} deleted during generation; stopping stream", sid);
                                    session_cancelled = true;
                                    break;
                                }
                            }
                            token_count += 1;
                            full_response.push_str(&token);
                            yield Ok::<Event, Infallible>(Event::default().data(token));
                        }
                        Err(e) => {
                            tracing::error!("Stream error: {:?}", e);
                            yield Ok::<Event, Infallible>(Event::default().data(format!("__ERROR__:{}", e)));
                        }
                    }
                }

                // Record metrics
                let duration = start_time.elapsed().as_secs_f64();
                histogram!("chat_inference_duration_seconds", duration);
                counter!("chat_generated_tokens_total", token_count);

                // Calculate tokens per second
                if duration > 0.0 {
                    let tokens_per_second = token_count as f64 / duration;
                    histogram!("chat_tokens_per_second", tokens_per_second);
                }

                // Save assistant response to history
                if let Some(ref sid) = sid_clone {
                    if session_cancelled {
                        tracing::info!("Skipping persistence for deleted session {}", sid);
                    } else {
                        let mut guard = sessions.lock().await;
                        if let Some(hist) = guard.get_mut(sid) {
                            hist.push(ChatMessage {
                                role: "assistant".to_string(),
                                content: full_response,
                            });
                        }
                        // Save state after assistant message
                        drop(guard); // release lock before saving
                        state_clone.persist_session(sid).await;
                    }
                }
            };

            // Convert mapped stream into axum::response::sse::Sse
            let keepalive = KeepAlive::new().interval(std::time::Duration::from_secs(15));
            let sse = Sse::new(wrapped_stream).keep_alive(keepalive);
            sse.into_response()
        }
        Err(e) => {
            tracing::error!("Inference error: {:?}", e);
            increment_counter!("chat_completions_errors_total");
            let body = serde_json::json!({"error": e.to_string()});
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
        }
    }
}

async fn chat_ws(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    // Wait for the first message which should be the config
    if let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            if let Ok(mut req) = serde_json::from_str::<InferenceRequest>(&text) {
                // Handle Session for WS
                let session_id = req.session_id.clone();
                if let Some(sid) = &session_id {
                    let mut sessions = state.sessions.lock().await;
                    let history = sessions.entry(sid.clone()).or_insert_with(|| {
                        vec![ChatMessage {
                            role: "system".to_string(),
                            content: "You are a helpful AI assistant.".to_string(),
                        }]
                    });

                    history.push(ChatMessage {
                        role: "user".to_string(),
                        content: req.prompt.clone(),
                    });

                    // Prune history
                    prune_history(history);

                    req.messages = Some(history.clone());

                    tracing::info!("Session {}: History length = {}", sid, history.len());
                    for (i, msg) in history.iter().enumerate() {
                        tracing::info!("  [{}] {}: {}", i, msg.role, msg.content);
                    }
                }
                if let Some(sid) = session_id.as_ref() {
                    state.persist_session(sid).await;
                }

                // Run inference
                if let Ok(mut stream) = state.run_inference_guarded(req).await {
                    let mut full_response = String::new();
                    let mut session_cancelled = false;

                    while let Some(result) = stream.next().await {
                        match result {
                            Ok(token) => {
                                if let Some(ref sid) = session_id {
                                    let session_still_exists = {
                                        let guard = state.sessions.lock().await;
                                        guard.contains_key(sid)
                                    };
                                    if !session_still_exists {
                                        tracing::info!("Session {} deleted during generation; closing websocket stream", sid);
                                        session_cancelled = true;
                                        break;
                                    }
                                }
                                full_response.push_str(&token);
                                if socket.send(Message::Text(token)).await.is_err() {
                                    break;
                                }
                            }
                            Err(e) => {
                                let _ =
                                    socket.send(Message::Text(format!("__ERROR__:{}", e))).await;
                                break;
                            }
                        }
                    }

                    // Save assistant response
                    if let Some(ref sid) = session_id {
                        if session_cancelled {
                            tracing::info!("Skipping persistence for deleted session {}", sid);
                        } else {
                            let mut guard = state.sessions.lock().await;
                            if let Some(hist) = guard.get_mut(sid) {
                                hist.push(ChatMessage {
                                    role: "assistant".to_string(),
                                    content: full_response,
                                });
                            }
                            drop(guard);
                            state.persist_session(sid).await;
                        }
                    }
                } else {
                    let _ = socket
                        .send(Message::Text(
                            "__ERROR__:Failed to start inference".to_string(),
                        ))
                        .await;
                }
            } else {
                let _ = socket
                    .send(Message::Text("__ERROR__:Invalid JSON request".to_string()))
                    .await;
            }
        }
    }
}
