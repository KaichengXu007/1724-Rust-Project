use axum::{extract::{State, Path}, routing::{get, post}, Router, response::IntoResponse, Json};
use axum::response::sse::{Sse, KeepAlive, Event};
use axum::extract::ws::{WebSocketUpgrade, WebSocket, Message};
use futures_util::StreamExt;
use crate::state::AppState;
use crate::models::{InferenceRequest, ModelsList, ChatMessage};
use std::convert::Infallible;
use metrics::{increment_counter, histogram, counter};
use std::time::Instant;

const MAX_HISTORY_LENGTH: usize = 20; // Keep last 20 messages (approx 10 rounds)

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/models", get(get_models))
        .route("/chat/completions", post(chat_completions))
        .route("/chat/ws", get(chat_ws))
        .route("/chat/history/:session_id", get(get_history))
        .route("/health", get(health_check))
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

async fn get_history(
    State(state): State<AppState>,
    Path(session_id): Path<String>
) -> impl IntoResponse {
    increment_counter!("history_requests_total");
    let sessions = state.sessions.lock().await;
    let history = sessions.get(&session_id).cloned().unwrap_or_default();
    Json(history)
}

async fn chat_completions(State(state): State<AppState>, Json(mut req): Json<InferenceRequest>) -> axum::response::Response {
    increment_counter!("chat_completions_requests_total");
    let start_time = Instant::now();

    // Handle Session: if session_id is present, append prompt to history and use history as context
    let session_id = req.session_id.clone();
    if let Some(sid) = &session_id {
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
    // Save state after user message
    state.save_sessions().await;

    // call engine to get TokenStream
    match state.engine.run_streaming_inference(req).await {
        Ok(mut stream) => {
            let sessions = state.sessions.clone();
            let sid_clone = session_id.clone();
            let state_clone = state.clone();

            // Wrap the stream to capture the full response
            let wrapped_stream = async_stream::stream! {
                let mut full_response = String::new();
                let mut token_count = 0;
                
                while let Some(result) = stream.next().await {
                    match result {
                        Ok(token) => {
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

                // Save assistant response to history
                if let Some(sid) = sid_clone {
                    let mut guard = sessions.lock().await;
                    if let Some(hist) = guard.get_mut(&sid) {
                        hist.push(ChatMessage {
                            role: "assistant".to_string(),
                            content: full_response,
                        });
                    }
                    // Save state after assistant message
                    drop(guard); // release lock before saving
                    state_clone.save_sessions().await;
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

async fn chat_ws(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
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
                // Save state after user message
                state.save_sessions().await;

                // Run inference
                if let Ok(mut stream) = state.engine.run_streaming_inference(req).await {
                    let mut full_response = String::new();
                    
                    while let Some(result) = stream.next().await {
                        match result {
                            Ok(token) => {
                                full_response.push_str(&token);
                                if socket.send(Message::Text(token)).await.is_err() {
                                    break;
                                }
                            }
                            Err(e) => {
                                let _ = socket.send(Message::Text(format!("__ERROR__:{}", e))).await;
                                break;
                            }
                        }
                    }
                    
                    // Save assistant response
                    if let Some(sid) = session_id {
                        let mut guard = state.sessions.lock().await;
                        if let Some(hist) = guard.get_mut(&sid) {
                            hist.push(ChatMessage {
                                role: "assistant".to_string(),
                                content: full_response,
                            });
                        }
                        drop(guard);
                        state.save_sessions().await;
                    }
                    
                } else {
                     let _ = socket.send(Message::Text("__ERROR__:Failed to start inference".to_string())).await;
                }
            } else {
                 let _ = socket.send(Message::Text("__ERROR__:Invalid JSON request".to_string())).await;
            }
        }
    }
}
