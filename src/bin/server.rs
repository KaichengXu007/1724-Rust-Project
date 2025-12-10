use axum::Server;
use llm_inference::config::Config;
use llm_inference::engine::M1EngineAdapter;
use llm_inference::routes;
use llm_inference::state::AppState;
use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration
    let config = Config::load();

    // Initialize logging
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&config.server.log_level));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    info!("🚀 Starting Rust LLM Inference Service");
    info!("📝 Configuration loaded");

    // Initialize Prometheus Metrics
    if config.observability.enable_metrics {
        let builder = PrometheusBuilder::new();
        let handle = builder
            .install_recorder()
            .expect("failed to install Prometheus recorder");
        info!(
            "📊 Prometheus metrics initialized at {}",
            config.observability.metrics_path
        );

        info!("🤖 Initializing Inference Engine...");

        // Load available models from config
        let available_models = config.models.available_models.clone();
        let model_labels: Vec<String> = available_models
            .iter()
            .map(|m| format!("{} ({})", m.name, m.id))
            .collect();

        info!("📦 Available models: {:?}", model_labels);

        let engine = Arc::new(M1EngineAdapter::new(available_models.clone()));

        // Pre-warm all models
        let device = if cfg!(feature = "cuda") {
            "cuda"
        } else {
            "cpu"
        };
        info!(
            "🔥 Pre-warming {} models on {}",
            available_models.len(),
            device
        );
        for model in &available_models {
            info!("🔥 Loading model: {} ({})", model.name, model.id);
            if let Err(e) = engine.warmup(&model.id, device).await {
                tracing::warn!("⚠️ Failed to pre-warm model {}: {:?}", model.name, e);
            } else {
                info!("✅ Model cached: {}", model.name);
            }
        }

        // Initialize AppState
        let state = AppState::new(engine, handle, config.clone()).await?;

        // Setup CORS
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        // Build router and attach rate-limit middleware (uses AppState clone)
        // Build router
        // Attach global rate-limit middleware so all routes (including /sessions)
        // receive X-RateLimit headers and 429 when exceeded.
        let app = routes::router()
            .route_layer(axum::middleware::from_fn_with_state(state.clone(), routes::rate_limit))
            .with_state(state)
            .layer(cors)
            .fallback_service(ServeDir::new("frontend/dist"));

        // Bind and serve
        let addr = SocketAddr::from((
            config
                .server
                .host
                .parse::<std::net::IpAddr>()
                .unwrap_or(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1))),
            config.server.port,
        ));

        info!("🌐 Server listening on http://{}", addr);
        info!("💬 Web UI available at http://{}", addr);
        if config.security.enable_auth {
            info!("🔐 API authentication enabled");
        }

        Server::bind(&addr).serve(app.into_make_service()).await?;
    } else {
        anyhow::bail!("Metrics must be enabled");
    }

    Ok(())
}
