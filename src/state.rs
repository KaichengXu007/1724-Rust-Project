use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use crate::engine::InferenceEngine;
use crate::models::ChatMessage;
use std::fs::File;
use std::io::BufReader;
use anyhow::Result;
use metrics_exporter_prometheus::PrometheusHandle;

const SESSIONS_FILE: &str = "sessions.json";

#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<dyn InferenceEngine>,
    pub sessions: Arc<Mutex<HashMap<String, Vec<ChatMessage>>>>,
    pub metrics_handle: PrometheusHandle,
}

impl AppState {
    pub fn new(engine: Arc<dyn InferenceEngine>, metrics_handle: PrometheusHandle) -> Self {
        let sessions = Self::load_sessions().unwrap_or_default();
        Self { 
            engine,
            sessions: Arc::new(Mutex::new(sessions)),
            metrics_handle,
        }
    }

    fn load_sessions() -> Result<HashMap<String, Vec<ChatMessage>>> {
        if std::path::Path::new(SESSIONS_FILE).exists() {
            let file = File::open(SESSIONS_FILE)?;
            let reader = BufReader::new(file);
            let sessions = serde_json::from_reader(reader)?;
            Ok(sessions)
        } else {
            Ok(HashMap::new())
        }
    }

    pub async fn save_sessions(&self) {
        let sessions = self.sessions.lock().await;
        if let Ok(file) = File::create(SESSIONS_FILE) {
            let _ = serde_json::to_writer(file, &*sessions);
        }
    }
}
