use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub models: ModelsConfig,
    pub security: SecurityConfig,
    pub limits: LimitsConfig,
    pub observability: ObservabilityConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelsConfig {
    #[serde(default)]
    pub model_dir: Option<PathBuf>,
    pub available_models: Vec<ModelConfig>,
    #[serde(default = "default_device")]
    pub default_device: String,
    #[serde(default = "default_max_concurrent")]
    pub max_concurrent_requests: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelConfig {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub path: Option<PathBuf>,
    #[serde(default)]
    pub quantization: Option<String>,
    #[serde(default)]
    pub context_length: Option<usize>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SecurityConfig {
    #[serde(default)]
    pub enable_auth: bool,
    #[serde(default)]
    pub api_keys: Vec<ApiKeyConfig>,
    #[serde(default)]
    pub allowed_origins: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiKeyConfig {
    pub key: String,
    pub name: String,
    #[serde(default)]
    pub rate_limit_per_minute: Option<u32>,
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LimitsConfig {
    #[serde(default = "default_max_prompt_length")]
    pub max_prompt_length: usize,
    #[serde(default = "default_max_response_tokens")]
    pub max_response_tokens: usize,
    #[serde(default = "default_max_sessions")]
    pub max_sessions: usize,
    #[serde(default = "default_session_ttl")]
    pub session_ttl_seconds: u64,
    #[serde(default = "default_rate_limit")]
    pub default_rate_limit_per_minute: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ObservabilityConfig {
    #[serde(default = "default_true")]
    pub enable_metrics: bool,
    #[serde(default = "default_true")]
    pub enable_tracing: bool,
    #[serde(default)]
    pub metrics_path: String,
}

// Default value functions
fn default_host() -> String {
    "127.0.0.1".to_string()
}
fn default_port() -> u16 {
    3000
}
fn default_log_level() -> String {
    "info".to_string()
}
fn default_device() -> String {
    "cuda".to_string()
}
fn default_max_concurrent() -> usize {
    10
}
fn default_max_prompt_length() -> usize {
    8192
}
fn default_max_response_tokens() -> usize {
    2048
}
fn default_max_sessions() -> usize {
    1000
}
fn default_session_ttl() -> u64 {
    3600
}
fn default_rate_limit() -> u32 {
    60
}
fn default_true() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: default_host(),
                port: default_port(),
                log_level: default_log_level(),
            },
            models: ModelsConfig {
                model_dir: None,
                available_models: vec![
                    ModelConfig {
                        id: "qwen".to_string(),
                        name: "Qwen/Qwen2.5-0.5B-Instruct".to_string(),
                        path: None,
                        quantization: None,
                        context_length: Some(4096),
                    },
                    ModelConfig {
                        id: "phi".to_string(),
                        name: "microsoft/Phi-3.5-mini-instruct".to_string(),
                        path: None,
                        quantization: None,
                        context_length: Some(4096),
                    },
                ],
                default_device: default_device(),
                max_concurrent_requests: default_max_concurrent(),
            },
            security: SecurityConfig {
                enable_auth: false,
                api_keys: vec![],
                allowed_origins: vec!["*".to_string()],
            },
            limits: LimitsConfig {
                max_prompt_length: default_max_prompt_length(),
                max_response_tokens: default_max_response_tokens(),
                max_sessions: default_max_sessions(),
                session_ttl_seconds: default_session_ttl(),
                default_rate_limit_per_minute: default_rate_limit(),
            },
            observability: ObservabilityConfig {
                enable_metrics: true,
                enable_tracing: true,
                metrics_path: "/metrics".to_string(),
            },
        }
    }
}

impl Config {
    /// Load configuration from a TOML file
    pub fn from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .context(format!("Failed to read config file: {}", path))?;
        let config: Config = toml::from_str(&content).context("Failed to parse config file")?;
        config.validate()?;
        Ok(config)
    }

    /// Load configuration with fallback to default
    pub fn load() -> Self {
        match Self::from_file("config.toml") {
            Ok(config) => {
                tracing::info!("✅ Loaded configuration from config.toml");
                config
            }
            Err(e) => {
                tracing::warn!("⚠️ Failed to load config.toml: {}. Using defaults.", e);
                Self::default()
            }
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.server.port == 0 {
            anyhow::bail!("Server port cannot be 0");
        }

        if self.models.available_models.is_empty() {
            anyhow::bail!("At least one model must be configured");
        }

        if self.security.enable_auth && self.security.api_keys.is_empty() {
            anyhow::bail!("Authentication enabled but no API keys configured");
        }

        Ok(())
    }

    /// Save configuration to file
    pub fn save(&self, path: &str) -> Result<()> {
        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;
        std::fs::write(path, content).context(format!("Failed to write config file: {}", path))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.server.port, 3000);
        assert!(!config.security.enable_auth);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        config.security.enable_auth = true;
        assert!(config.validate().is_err());
    }
}
