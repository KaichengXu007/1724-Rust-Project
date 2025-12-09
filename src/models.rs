use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Inference 请求结构，字段来源于原始的 parse::Args
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct InferenceRequest {
    pub model_name: String,
    pub model_dir: Option<PathBuf>,
    pub prompt: String,
    #[serde(default)]
    pub messages: Option<Vec<ChatMessage>>,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default = "default_max_token")]
    pub max_token: usize,
    #[serde(default = "default_temperature")]
    pub temperature: f64,
    #[serde(default = "default_top_p")]
    pub top_p: f64,
    #[serde(default = "default_top_k")]
    pub top_k: i32,
    #[serde(default = "default_repeat_penalty")]
    pub repeat_penalty: f32,
    #[serde(default)]
    pub stop: Vec<String>,
    #[serde(default = "default_device")]
    pub device: String,
}

/// Completion request (non-chat, raw completion)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CompletionRequest {
    pub model: String,
    pub prompt: String,
    #[serde(default = "default_max_token")]
    pub max_tokens: usize,
    #[serde(default = "default_temperature")]
    pub temperature: f64,
    #[serde(default = "default_top_p")]
    pub top_p: f64,
    #[serde(default)]
    pub stop: Vec<String>,
    #[serde(default)]
    pub stream: bool,
}

fn default_max_token() -> usize {
    128
}
fn default_temperature() -> f64 {
    0.7
}
fn default_top_p() -> f64 {
    0.95
}
fn default_top_k() -> i32 {
    10
}
fn default_repeat_penalty() -> f32 {
    1.0
}
fn default_device() -> String {
    "cpu".to_string()
}

/// 标准 API 返回的模型列表包装
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelsList {
    pub models: Vec<String>,
}
