use axum::Server;
use std::net::SocketAddr;
use std::sync::Arc;
use llm_inference::engine::M1EngineAdapter;
use llm_inference::routes;
use llm_inference::state::AppState;
use llm_inference::config::Config;
use tracing::info;
use tower_http::services::ServeDir;
use tower_http::cors::{CorsLayer, Any};
use metrics_exporter_prometheus::PrometheusBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration
    let config = Config::load();
    
    // Initialize logging
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&config.server.log_level));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();

    info!("🚀 Starting Rust LLM Inference Service");
    info!("📝 Configuration loaded");

    // Initialize Prometheus Metrics
    if config.observability.enable_metrics {
        let builder = PrometheusBuilder::new();
        let handle = builder.install_recorder()
            .expect("failed to install Prometheus recorder");
        info!("📊 Prometheus metrics initialized at {}", config.observability.metrics_path);

        info!("🤖 Initializing Inference Engine...");
        
        // Load available models from config
        let available_models: Vec<String> = config.models.available_models
            .iter()
            .map(|m| m.name.clone())
            .collect();
        
        info!("📦 Available models: {:?}", available_models);
        
        let engine = Arc::new(M1EngineAdapter::new(available_models));

        // Initialize AppState
        let state = AppState::new(engine, handle, config.clone());

        // Setup CORS
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        // Build router
        let app = routes::router()
            .with_state(state)
            .layer(cors)
            .fallback_service(ServeDir::new("public"));

        // Bind and serve
        let addr = SocketAddr::from((
            config.server.host.parse::<std::net::IpAddr>()
                .unwrap_or(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1))),
            config.server.port
        ));
        
        info!("🌐 Server listening on http://{}", addr);
        info!("💬 Web UI available at http://{}", addr);
        if config.security.enable_auth {
            info!("🔐 API authentication enabled");
        }
        
        Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;
    } else {
        anyhow::bail!("Metrics must be enabled");
    }

    Ok(())
}
