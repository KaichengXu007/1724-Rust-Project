use crate::config::Config;
use crate::engine::{InferenceEngine, TokenStream};
use crate::models::{ChatMessage, InferenceRequest};
use anyhow::{anyhow, Result};
use async_stream::stream;
use futures_util::{FutureExt, StreamExt};
use metrics_exporter_prometheus::PrometheusHandle;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use sqlx::Row;
use std::any::Any;
use std::collections::HashMap;
use std::panic::AssertUnwindSafe;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, warn};

const SESSIONS_DB: &str = "sessions.db";

struct SessionStore {
    pool: SqlitePool,
}

impl SessionStore {
    async fn new(db_path: &str) -> Result<Self> {
        let connect_opts = SqliteConnectOptions::new()
            .filename(Path::new(db_path))
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(connect_opts)
            .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS sessions (
                session_id TEXT PRIMARY KEY,
                history TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    async fn load_sessions(&self) -> Result<HashMap<String, Vec<ChatMessage>>> {
        let mut map = HashMap::new();
        let rows = sqlx::query("SELECT session_id, history FROM sessions")
            .fetch_all(&self.pool)
            .await?;

        for row in rows {
            let session_id: String = row.try_get("session_id")?;
            let history_json: String = row.try_get("history")?;
            match serde_json::from_str::<Vec<ChatMessage>>(&history_json) {
                Ok(history) => {
                    map.insert(session_id, history);
                }
                Err(err) => {
                    warn!("Failed to deserialize history for {}: {}", session_id, err);
                }
            }
        }

        Ok(map)
    }

    async fn upsert_session(&self, session_id: &str, history: &[ChatMessage]) -> Result<()> {
        let payload = serde_json::to_string(history)?;
        sqlx::query(
            "INSERT INTO sessions (session_id, history) VALUES (?, ?)
             ON CONFLICT(session_id) DO UPDATE SET history = excluded.history",
        )
        .bind(session_id)
        .bind(payload)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete_session(&self, session_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM sessions WHERE session_id = ?")
            .bind(session_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn replace_all(&self, snapshot: &HashMap<String, Vec<ChatMessage>>) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("DELETE FROM sessions")
            .execute(&mut *tx)
            .await?;

        for (session_id, history) in snapshot.iter() {
            let payload = serde_json::to_string(history)?;
            sqlx::query(
                "INSERT INTO sessions (session_id, history) VALUES (?, ?)
                 ON CONFLICT(session_id) DO UPDATE SET history = excluded.history",
            )
            .bind(session_id)
            .bind(payload)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<dyn InferenceEngine>,
    pub sessions: Arc<Mutex<HashMap<String, Vec<ChatMessage>>>>,
    pub metrics_handle: PrometheusHandle,
    pub config: Arc<Config>,
    session_store: Arc<SessionStore>,
}

impl AppState {
    pub async fn new(
        engine: Arc<dyn InferenceEngine>,
        metrics_handle: PrometheusHandle,
        config: Config,
    ) -> Result<Self> {
        let store = Arc::new(SessionStore::new(SESSIONS_DB).await?);
        let sessions = store.load_sessions().await.unwrap_or_default();

        Ok(Self {
            engine,
            sessions: Arc::new(Mutex::new(sessions)),
            metrics_handle,
            config: Arc::new(config),
            session_store: store,
        })
    }

    pub async fn save_sessions(&self) {
        let snapshot = {
            let sessions = self.sessions.lock().await;
            sessions.clone()
        };

        if let Err(err) = self.session_store.replace_all(&snapshot).await {
            error!("Failed to persist sessions snapshot: {}", err);
        }
    }

    pub async fn persist_session(&self, session_id: &str) {
        let history = {
            let sessions = self.sessions.lock().await;
            sessions.get(session_id).cloned()
        };

        if let Some(history) = history {
            if let Err(err) = self
                .session_store
                .upsert_session(session_id, &history)
                .await
            {
                error!("Failed to persist session {}: {}", session_id, err);
            }
        }
    }

    pub async fn delete_session_record(&self, session_id: &str) {
        if let Err(err) = self.session_store.delete_session(session_id).await {
            error!("Failed to delete session {}: {}", session_id, err);
        }
    }

    /// Validate prompt length against configured limits
    pub fn validate_prompt_length(&self, prompt: &str) -> Result<()> {
        if prompt.len() > self.config.limits.max_prompt_length {
            anyhow::bail!(
                "Prompt exceeds maximum length of {} characters",
                self.config.limits.max_prompt_length
            );
        }
        Ok(())
    }

    /// Check session limit
    pub async fn check_session_limit(&self) -> Result<()> {
        let sessions = self.sessions.lock().await;
        if sessions.len() >= self.config.limits.max_sessions {
            anyhow::bail!(
                "Maximum number of sessions ({}) reached",
                self.config.limits.max_sessions
            );
        }
        Ok(())
    }

    pub async fn run_inference_guarded(&self, req: InferenceRequest) -> Result<TokenStream> {
        let fut = AssertUnwindSafe(self.engine.run_streaming_inference(req));
        match fut.catch_unwind().await {
            Ok(result) => result.map(Self::guard_stream),
            Err(payload) => {
                let reason = panic_message(payload);
                error!("Inference engine panicked: {}", reason);
                Err(anyhow!("Inference engine panicked"))
            }
        }
    }

    fn guard_stream(stream: TokenStream) -> TokenStream {
        Box::pin(stream! {
            let mut inner = stream;
            loop {
                let next = AssertUnwindSafe(inner.next()).catch_unwind().await;
                match next {
                    Ok(Some(item)) => {
                        yield item;
                    }
                    Ok(None) => break,
                    Err(payload) => {
                        let reason = panic_message(payload);
                        error!("Inference stream panicked: {}", reason);
                        yield Err(anyhow!("Inference engine panicked"));
                        break;
                    }
                }
            }
        })
    }
}

fn panic_message(payload: Box<dyn Any + Send>) -> String {
    if let Some(msg) = payload.downcast_ref::<&str>() {
        msg.to_string()
    } else if let Some(msg) = payload.downcast_ref::<String>() {
        msg.clone()
    } else {
        "unknown panic".to_string()
    }
}
