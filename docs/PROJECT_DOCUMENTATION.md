# Rust LLM Inference Service

Complete documentation for the Rust LLM Inference Service - a production-ready, OpenAI-compatible LLM inference server with GPU acceleration.

---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Getting Started](#getting-started)
- [Configuration](#configuration)
- [Architecture](#architecture)
- [Deployment](#deployment)
- [Development](#development)
- [Testing](#testing)
- [Monitoring](#monitoring)
- [Troubleshooting](#troubleshooting)

---

## Overview

The Rust LLM Inference Service is a high-performance, production-ready server for running large language models locally with GPU acceleration. It provides OpenAI-compatible APIs, real-time streaming, session management, and enterprise-grade security features.

**Key Highlights**:
- 🚀 **GPU Accelerated**: CUDA support for NVIDIA GPUs (10-50x faster than CPU)
- 🔄 **Real-time Streaming**: WebSocket and SSE streaming for instant responses
- 💬 **Session Management**: Multi-session support with persistent conversation history
- 🔒 **Enterprise Security**: API key authentication, rate limiting, content validation
- 📊 **Observability**: Prometheus metrics, health checks, structured logging
- 🐳 **Cloud Native**: Docker containers with GPU support
- 🎨 **Modern Web UI**: Built-in chat interface with advanced parameter controls

---

## Features

### Core Capabilities

#### 1. High-Performance Inference
- **mistral.rs Engine**: Industry-leading Rust inference engine
- **GPU Acceleration**: CUDA support for NVIDIA GPUs
- **CPU Fallback**: Automatic fallback when GPU unavailable
- **Metal Support**: macOS GPU acceleration
- **Lazy Loading**: Models load on first request and cached
- **Multiple Models**: Support for Qwen, Phi-3.5, and compatible models

#### 2. Web Service & APIs
- **REST API**: `/completions` and `/chat/completions` endpoints
- **WebSocket**: Real-time bidirectional streaming at `/chat/ws`
- **SSE Streaming**: Server-Sent Events for HTTP streaming
- **OpenAI Compatible**: Drop-in replacement for OpenAI API
- **Full Parameter Control**:
  - Temperature (0-2)
  - Top-P nucleus sampling
  - Top-K sampling
  - Max tokens
  - Repeat penalty
  - System prompts
  - Stop sequences

#### 3. Session Management
- **Multi-Session**: Independent conversation threads
- **Persistent Storage**: History saved to `sessions.json`
- **Auto-Trimming**: Keep last 20 messages to prevent context overflow
- **History API**: Query and manage conversation history
- **Session Rollback**: Rewind conversations to previous states

#### 4. Modern Web Interface
- **Dark Mode**: Professional dark theme
- **Markdown Support**: Rich text formatting and code highlighting
- **Advanced Settings Panel**: Full control over generation parameters
- **Live Statistics**: Token counter with tokens/s display
- **Code Features**: Syntax highlighting and copy buttons
- **Message Controls**: Stop, regenerate, edit capabilities
- **Session Sidebar**: Create, switch, delete sessions
- **Export/Import**: Save and restore conversation history

#### 5. Security & Governance
- **API Key Authentication**: Bearer token support
- **Rate Limiting**: Per-key or per-IP request throttling
- **Content Validation**: Prompt and response length limits
- **CORS Support**: Configurable cross-origin policies
- **Input Sanitization**: Protection against malicious inputs

#### 6. Observability
- **Prometheus Metrics**: Comprehensive performance tracking
- **Health Checks**: `/health` and `/readiness` endpoints
- **Structured Logging**: Configurable log levels
- **Key Metrics**:
  - Request counts
  - Inference latency
  - Token generation rate
  - Error rates
  - Session statistics

#### 7. Deployment
- **Docker Support**: Multi-stage builds for CPU and GPU
- **Docker Compose**: Integrated stack with Prometheus and Grafana
- **Health Probes**: Container-level health checking
- **Volume Mounts**: Persistent models and data
- **Environment Config**: Override settings via env vars

---

## Getting Started

### Prerequisites

- **Rust**: 1.75+ ([Install Rust](https://rustup.rs/))
- **Git**: For cloning repository
- **(Optional)** NVIDIA GPU with CUDA 12.1+ for GPU acceleration
- **(Optional)** Docker for containerized deployment

### Quick Start

#### Option 1: Run from Source

```bash
# Clone repository
git clone https://github.com/yourusername/rust-llm-inference.git
cd rust-llm-inference

# Run with GPU (recommended)
cargo run --release --features cuda --bin server

# OR run with CPU only
cargo run --release --bin server

# Open browser
# Navigate to http://localhost:3000
```

#### Option 2: Docker

```bash
# Clone repository
git clone https://github.com/yourusername/rust-llm-inference.git
cd rust-llm-inference

# Run with GPU
docker-compose up llm-gpu

# OR run with CPU
docker-compose up llm-cpu

# Open browser
# Navigate to http://localhost:3000
```

The service will:
- Start on port 3000
- Load models on first request
- Use sensible defaults
- Create sessions automatically

### First Request

Test with cURL:
```bash
curl -X POST http://localhost:3000/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "Qwen/Qwen2.5-0.5B-Instruct",
    "prompt": "Hello, world!",
    "max_tokens": 50
  }'
```

---

## Configuration

### Configuration File

Create `config.toml` (optional - defaults work out of the box):

```toml
[server]
host = "0.0.0.0"
port = 3000
log_level = "info"  # trace, debug, info, warn, error

[models]
# Available models
models = [
    "Qwen/Qwen2.5-0.5B-Instruct",
    "microsoft/Phi-3.5-mini-instruct"
]
default_device = "cuda"  # cuda, cpu, metal
default_quantization = "bf16"

[security]
enable_auth = false
api_keys = [
    { key = "sk-your-key-here", rate_limit = 100, enabled = true }
]
enable_cors = true
cors_origins = ["*"]

[limits]
max_prompt_length = 8192
max_response_tokens = 2048
max_sessions = 1000
session_ttl_seconds = 86400  # 24 hours
default_rate_limit = 60

[observability]
enable_metrics = true
metrics_path = "/metrics"
enable_tracing = false
```

### Environment Variables

Override config via environment:

```bash
# Server
export RUST_LOG=info
export SERVER_PORT=8080

# Security
export ENABLE_AUTH=true
export API_KEY=sk-your-secret-key

# Models
export DEFAULT_DEVICE=cuda
```

---

## Architecture

### System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Client Layer                         │
│  (Web Browser, cURL, Python, JavaScript, Postman, etc.)    │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                      Axum Web Server                        │
│  ┌──────────────────────────────────────────────────────┐  │
│  │              Middleware Stack                        │  │
│  │  • Authentication (API Keys)                         │  │
│  │  • Rate Limiting (Per-key/IP)                        │  │
│  │  • CORS                                              │  │
│  │  • Logging & Tracing                                 │  │
│  └──────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │                   Routes                             │  │
│  │  • /health, /readiness, /metrics                     │  │
│  │  • /models, /completions                             │  │
│  │  • /chat/completions, /chat/ws                       │  │
│  │  • /sessions, /chat/history                          │  │
│  └──────────────────────────────────────────────────────┘  │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                      State Layer                            │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Arc<AppState>                                       │  │
│  │    • M1EngineAdapter (Thread-safe model cache)       │  │
│  │    • SessionStore (Arc<Mutex<HashMap>>)              │  │
│  │    • Config                                          │  │
│  │    • Metrics Registry                                │  │
│  └──────────────────────────────────────────────────────┘  │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                   Inference Engine                          │
│  ┌──────────────────────────────────────────────────────┐  │
│  │         mistral.rs (M1EngineAdapter)                 │  │
│  │  • Model loading & caching                           │  │
│  │  • GPU/CPU device management                         │  │
│  │  • Token generation                                  │  │
│  │  • Streaming via async channels                      │  │
│  └──────────────────────────────────────────────────────┘  │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                Hardware / Runtime                           │
│  • CUDA (NVIDIA GPU)                                        │
│  • CPU (Fallback)                                           │
│  • Metal (macOS GPU)                                        │
└─────────────────────────────────────────────────────────────┘
```

### Key Components

#### 1. Engine Layer (`src/engine.rs`)
- `M1EngineAdapter`: Wrapper around mistral.rs
- Model caching: HashMap of loaded models
- Lazy loading: Models load on first request
- Device management: CUDA/CPU/Metal selection

#### 2. Routes Layer (`src/routes.rs`)
- REST endpoints: `/completions`, `/chat/completions`
- WebSocket: `/chat/ws` for streaming
- Session APIs: `/sessions`, `/chat/history`
- Health & metrics: `/health`, `/readiness`, `/metrics`

#### 3. State Management (`src/state.rs`)
- `AppState`: Global application state
- Thread-safe: Uses `Arc` for sharing across threads
- Session storage: `Arc<Mutex<HashMap<SessionId, Session>>>`
- Metrics registry: Prometheus collectors

#### 4. Models (`src/models.rs`)
- Request/response structures
- Validation logic
- Serialization/deserialization

#### 5. Configuration (`src/config.rs`)
- TOML parsing
- Environment variable overrides
- Validation and defaults

#### 6. Middleware
- Authentication: Bearer token validation
- Rate limiting: Token bucket algorithm
- CORS: Cross-origin policy enforcement
- Logging: Request/response tracing

---

## Deployment

### Local Development

```bash
# Development mode (fast compilation)
cargo run --bin server

# Release mode (optimized)
cargo run --release --bin server

# With CUDA
cargo run --release --features cuda --bin server
```

### Docker Production

**Build Images**:
```bash
# CPU version
docker build -t llm-service:cpu -f Dockerfile .

# GPU version
docker build -t llm-service:gpu -f Dockerfile.cuda .
```

**Run Containers**:
```bash
# CPU
docker run -p 3000:3000 \
  -v $(pwd)/models:/app/models \
  -v $(pwd)/sessions.json:/app/sessions.json \
  llm-service:cpu

# GPU
docker run --gpus all -p 3000:3000 \
  -v $(pwd)/models:/app/models \
  -v $(pwd)/sessions.json:/app/sessions.json \
  llm-service:gpu
```

### Docker Compose Stack

```bash
# Start full stack (app + Prometheus + Grafana)
docker-compose up -d

# View logs
docker-compose logs -f llm-gpu

# Stop stack
docker-compose down
```

**Services**:
- LLM Service: `http://localhost:3000`
- Prometheus: `http://localhost:9090`
- Grafana: `http://localhost:3001` (admin/admin)

### WSL2 with CUDA

For Windows users with NVIDIA GPUs:

```bash
# Enter WSL
wsl

# Navigate to project
cd /mnt/c/Users/YourName/path/to/project

# Build with CUDA
bash scripts/build_cuda_wsl.sh

# Run
./target/release/server
```

See `scripts/WSL_SETUP.md` for complete WSL setup guide.

---

## Development

### Project Structure

```
rust-llm-inference/
├── src/
│   ├── bin/
│   │   └── server.rs          # Binary entry point
│   ├── engine.rs              # Inference engine
│   ├── engine_mock.rs         # Mock for testing
│   ├── routes.rs              # API endpoints
│   ├── state.rs               # Application state
│   ├── models.rs              # Request/response models
│   └── lib.rs                 # Library exports
├── tests/
│   ├── integration_tests.rs   # API tests
│   ├── config_tests.rs        # Config tests
│   └── middleware_tests.rs    # Middleware tests
├── public/
│   └── index.html             # Web UI
├── docs/
│   ├── API_REFERENCE.md       # API documentation
│   └── PROJECT_DOCUMENTATION.md # This file
├── scripts/
│   ├── build_cuda_wsl.sh      # CUDA build script
│   ├── build_cpu_wsl.sh       # CPU build script
│   └── WSL_SETUP.md           # WSL setup guide
├── Cargo.toml                 # Dependencies
├── config.example.toml        # Example config
├── docker-compose.yml         # Docker stack
├── Dockerfile                 # CPU image
├── Dockerfile.cuda            # GPU image
├── prometheus.yml             # Metrics config
└── README.md                  # Quick start
```

### Adding a New Endpoint

1. **Define Model** (`src/models.rs`):
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct MyRequest {
    pub param: String,
}
```

2. **Add Route** (`src/routes.rs`):
```rust
async fn my_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<MyRequest>,
) -> Result<Json<MyResponse>, StatusCode> {
    // Implementation
}
```

3. **Register Route** (`src/bin/server.rs`):
```rust
let app = Router::new()
    .route("/my-endpoint", post(my_handler));
```

### Adding a New Model

1. **Update Config** (`config.toml`):
```toml
[models]
models = [
    "Qwen/Qwen2.5-0.5B-Instruct",
    "your-org/your-model"
]
```

2. **Model Auto-loads**: No code changes needed - lazy loading handles it

---

## Testing

### Run All Tests

```bash
# All tests
cargo test

# With output
cargo test -- --nocapture

# Specific test
cargo test test_completions_endpoint

# Integration tests only
cargo test --test integration_tests
```

### Test Coverage

- **Unit Tests**: Config, middleware, rate limiter
- **Integration Tests**: All API endpoints, session management
- **Mock Engine**: Fast tests without real models

### Manual Testing

Use provided Postman collection:
```bash
# Import postman_collection.json into Postman
# Configure environment variables
# Run collection
```

---

## Monitoring

### Prometheus Metrics

Access metrics at `http://localhost:3000/metrics`

**Key Metrics**:
- `health_check_requests_total`
- `completions_requests_total`
- `chat_completions_requests_total`
- `completions_duration_seconds` (histogram)
- `completions_tokens_total`
- `chat_generated_tokens_total`
- `completions_errors_total`

### Grafana Dashboards

1. Start stack: `docker-compose up -d`
2. Open Grafana: `http://localhost:3001`
3. Login: `admin` / `admin`
4. Add Prometheus datasource: `http://prometheus:9090`
5. Import dashboard or create custom

**Useful Queries**:
```promql
# Request rate
rate(completions_requests_total[5m])

# Average latency
rate(completions_duration_seconds_sum[5m]) / 
rate(completions_duration_seconds_count[5m])

# Tokens per second
rate(completions_tokens_total[5m])

# Error rate
rate(completions_errors_total[5m])
```

---

## Troubleshooting

### Common Issues

#### 1. CUDA Not Found
**Symptom**: `nvidia-smi: command not found`

**Solution**:
```bash
# Check CUDA installation
nvcc --version

# WSL: Install CUDA toolkit
sudo apt-get install -y cuda-toolkit-12-1

# Windows: Add to PATH
$env:PATH += ";C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.1\bin"
```

#### 2. Model Loading Failed
**Symptom**: `Failed to load model: File not found`

**Solution**:
- Check model name matches exactly
- Ensure internet connection (first load downloads)
- Check disk space for model cache
- Try CPU device if GPU fails

#### 3. Port Already in Use
**Symptom**: `Address already in use`

**Solution**:
```bash
# Change port in config.toml
[server]
port = 8080

# Or kill existing process
# Windows
netstat -ano | findstr :3000
taskkill /PID <PID> /F

# Linux/Mac
lsof -ti:3000 | xargs kill -9
```

#### 4. Slow Generation
**Symptom**: Tokens generate slowly

**Solution**:
- Enable CUDA: `--features cuda`
- Check GPU usage: `nvidia-smi`
- Reduce `max_tokens`
- Use smaller model (Qwen 0.5B)

#### 5. Out of Memory
**Symptom**: `CUDA out of memory`

**Solution**:
- Use CPU: Remove `--features cuda`
- Use smaller model
- Reduce `max_tokens`
- Close other GPU applications

### Debug Mode

```bash
# Enable debug logging
export RUST_LOG=debug
cargo run --bin server

# Trace all requests
export RUST_LOG=trace
cargo run --bin server
```

### Support

For additional help:
- Check API documentation: `docs/API_REFERENCE.md`
- Review example config: `config.example.toml`
- Run health check: `curl http://localhost:3000/health`
- Check metrics: `curl http://localhost:3000/metrics`

---

*Last Updated: 2025-12-07*
