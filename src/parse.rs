use anyhow::Result;
use serde::Deserialize;
use std::path::PathBuf;
#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]

pub struct Args {
    pub model_name: String,
    pub model_dir: PathBuf,
    pub prompt: String,
    #[serde(default = "default_max_token")]
    pub max_token: usize,
    #[serde(default = "default_temperature")]
    pub temperature: f64,
    #[serde(default = "default_top_p")]
    pub top_p: f64,
    #[serde(default = "default_top_k")]
    pub top_k: i32,
    pub repeat_penalty: f32,
    pub stop: Vec<String>,
    #[serde(default = "default_device")]
    pub device: String,
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
fn default_device() -> String {
    "cpu".to_string()
}

pub fn parse_args(json: &str) -> Result<Args> {
    let args: Args = serde_json::from_str(json)?;
    Ok(args)
}
