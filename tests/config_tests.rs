use llm_inference::config::*;

#[test]
fn test_default_config() {
    let config = Config::default();
    assert_eq!(config.server.port, 3000);
    assert_eq!(config.server.host, "127.0.0.1");
    assert!(!config.security.enable_auth);
    assert!(config.observability.enable_metrics);
}

#[test]
fn test_config_validation_success() {
    let config = Config::default();
    assert!(config.validate().is_ok());
}

#[test]
fn test_config_validation_auth_without_keys() {
    let mut config = Config::default();
    config.security.enable_auth = true;
    assert!(config.validate().is_err());
}

#[test]
fn test_config_validation_zero_port() {
    let mut config = Config::default();
    config.server.port = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_config_validation_no_models() {
    let mut config = Config::default();
    config.models.available_models.clear();
    assert!(config.validate().is_err());
}

#[test]
fn test_config_with_api_keys() {
    let mut config = Config::default();
    config.security.enable_auth = true;
    config.security.api_keys.push(ApiKeyConfig {
        key: "test-key".to_string(),
        name: "test".to_string(),
        rate_limit_per_minute: Some(100),
        enabled: true,
    });
    assert!(config.validate().is_ok());
}

#[test]
fn test_config_model_config() {
    let config = Config::default();
    assert_eq!(config.models.available_models.len(), 2);
    assert_eq!(config.models.default_device, "cuda");
}

#[test]
fn test_config_limits() {
    let config = Config::default();
    assert_eq!(config.limits.max_prompt_length, 8192);
    assert_eq!(config.limits.max_response_tokens, 2048);
    assert_eq!(config.limits.max_sessions, 1000);
}
