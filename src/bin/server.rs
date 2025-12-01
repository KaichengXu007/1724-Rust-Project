use axum::Server;
use std::net::SocketAddr;
use std::sync::Arc;
use llm_inference::engine::M1EngineAdapter;
use llm_inference::routes;
use llm_inference::state::AppState;
use tracing::info;
use tower_http::services::ServeDir;
use metrics_exporter_prometheus::PrometheusBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();

    // Initialize Prometheus Metrics
    let builder = PrometheusBuilder::new();
    let handle = builder.install_recorder().expect("failed to install Prometheus recorder");
    info!("Prometheus metrics initialized");

    info!("Initializing Inference Engine...");
    
    // Pre-define some popular models that work well with mistral.rs
    // These will be lazy-loaded upon first request
    let available_models = vec![
        "Qwen/Qwen2.5-0.5B-Instruct".to_string(),
        "microsoft/Phi-3.5-mini-instruct".to_string(),
    ];
    
    let engine = Arc::new(M1EngineAdapter::new(available_models));

    // Initialize AppState
    let state = AppState::new(engine, handle);

    // Build router
    let app = routes::router()
        .with_state(state)
        .fallback_service(ServeDir::new("public"));

    // Bind and serve
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Listening on http://{}", addr);
    
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
