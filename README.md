# 🚀 Rust LLM Inference Service

A high-performance, production-ready Large Language Model (LLM) inference service built entirely in Rust. Provides OpenAI-compatible APIs with token streaming, session management, and a modern web UI.

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](#-license)
[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](docker/)

## ✨ Features

### 🎯 Core Capabilities
- **Multiple Model Support**: Load and manage multiple GGUF-format models via Candle
- **Streaming Inference**: Real-time token streaming via Server-Sent Events (SSE) and WebSocket
- **Session Management**: Stateful conversations with configurable context limits
- **Modern Web UI**: Built-in chat interface with markdown rendering, dark mode, and message editing

### 🔒 Security & Governance
- **API Key Authentication**: Optional token-based authentication
- **Rate Limiting**: Per-key and IP-based rate limiting
- **Content Validation**: Configurable prompt/response length guards
- **CORS Support**: Cross-origin resource sharing configuration

### 📊 Observability
- **Prometheus Metrics**: Built-in metrics for latency, throughput, and token counts
- **Structured Logging**: Configurable log levels with tracing
- **Health Probes**: `/health` and `/readiness` endpoints for orchestration
- **Performance Tracking**: Inference time, tokens/second, cache hits

### 🚢 Deployment
- **Single Binary**: Portable executable with zero runtime dependencies
- **Docker Support**: Multi-stage builds for CPU and CUDA
- **Docker Compose**: Ready-to-use orchestration with Prometheus/Grafana
- **Configuration**: TOML-based config with sensible defaults

### 🔌 API Compatibility
- **OpenAI-style Endpoints**: `/completions`, `/chat/completions`
- **WebSocket Chat**: Real-time bidirectional streaming
- **Model Registry**: List, query, and manage models
- **RESTful Design**: Standard HTTP methods and status codes

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Web UI (HTML/JS)                       │
│                  Markdown • Dark Mode • SSE                 │
└────────────────────────┬────────────────────────────────────┘
                         │ HTTP/WS
┌────────────────────────▼────────────────────────────────────┐
│                    Axum Web Framework                       │
│            Routes • Middleware • State Management            │
├─────────────────────────────────────────────────────────────┤
│  Authentication  │  Rate Limiting  │  Content Validation    │
├─────────────────────────────────────────────────────────────┤
│               Session Manager (HashMap + Mutex)             │
│             Conversation History • Context Pruning           │
├─────────────────────────────────────────────────────────────┤
│              M1 Engine Adapter (mistral.rs)                 │
│         Model Loader • Tokenization • Sampling              │
├─────────────────────────────────────────────────────────────┤
│                   Candle ML Framework                       │
│              CUDA • Metal • CPU • Quantization              │
└─────────────────────────────────────────────────────────────┘
```

**Key Components**:
- **`routes.rs`**: HTTP endpoints, SSE/WebSocket handlers
- **`engine.rs`**: Inference abstraction, model management
- **`state.rs`**: Application state, session persistence
- **`middleware.rs`**: Auth, rate limiting, validation
- **`config.rs`**: TOML configuration parsing

---

## 🚀 Quick Start

### Prerequisites
- **Rust** 1.75+ (`rustup` recommended)
- **(Optional)** NVIDIA GPU + CUDA Toolkit 12.2+ for GPU acceleration
- **(Optional)** Docker for containerized deployment

### Installation

1. **Clone the repository**:
```bash
git clone https://github.com/yourusername/rust-llm-inference.git
cd rust-llm-inference
```

2. **Create configuration** (optional):
```bash
cp config.example.toml config.toml
# Edit config.toml to customize settings
```

3. **Run the service**:

**CPU Mode**:
```bash
cargo run --release --bin server
```

**GPU Mode (CUDA)**:
```bash
cargo run --release --features cuda --bin server
```

4. **Access the web UI**:
Open your browser to `http://localhost:3000`

---

## 📝 Configuration

The service uses a TOML configuration file (`config.toml`). See `config.example.toml` for full options.

### Key Settings

```toml
[server]
host = "127.0.0.1"
port = 3000
log_level = "info"

[models]
default_device = "cuda"  # cuda, cpu, metal
max_concurrent_requests = 10

[[models.available_models]]
id = "qwen"
name = "Qwen/Qwen2.5-0.5B-Instruct"
context_length = 4096

[security]
enable_auth = false  # Set true to require API keys

[limits]
max_prompt_length = 8192
max_response_tokens = 2048
max_sessions = 1000
default_rate_limit_per_minute = 60
```

### Environment Variables

- `RUST_LOG`: Override log level (e.g., `debug`, `trace`)
- `CUDA_VISIBLE_DEVICES`: Select GPU device (e.g., `0`)

---

## 🔌 API Usage

See [API Reference](docs/API_REFERENCE.md) for comprehensive documentation.

### Examples

**List models**:
```bash
curl http://localhost:3000/models
```

**Generate completion**:
```bash
curl -X POST http://localhost:3000/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "Qwen/Qwen2.5-0.5B-Instruct",
    "prompt": "Explain Rust ownership in one sentence:",
    "max_tokens": 50
  }'
```

**Chat with streaming**:
```bash
curl -X POST http://localhost:3000/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model-name": "Qwen/Qwen2.5-0.5B-Instruct",
    "prompt": "What is async/await?",
    "max-token": 256,
    "device": "cuda"
  }'
```

**Python client**:
```python
import requests

response = requests.post(
    "http://localhost:3000/completions",
    json={
        "model": "Qwen/Qwen2.5-0.5B-Instruct",
        "prompt": "Hello, Rust!",
        "max_tokens": 50
    }
)
print(response.json()["text"])
```

---

## 🐳 Docker Deployment

### CPU Version

```bash
docker build -t rust-llm:cpu -f docker/Dockerfile .
docker run -p 3000:3000 -v ./config.toml:/app/config.toml rust-llm:cpu
```

### GPU Version (CUDA)

```bash
docker build -f docker/Dockerfile.cuda -t rust-llm:cuda .
docker run --gpus all -p 3000:3000 rust-llm:cuda
```

### Docker Compose

```bash
# CPU service
docker-compose -f docker/docker-compose.yml up llm-cpu

# GPU service
docker-compose -f docker/docker-compose.yml up llm-gpu

# With Prometheus + Grafana
docker-compose -f docker/docker-compose.yml up
```

Access services:
- **LLM Service**: http://localhost:3000
- **Prometheus**: http://localhost:9090
- **Grafana**: http://localhost:3002 (admin/admin)

---

## 🧪 Testing

Run the test suite:

```bash
# All tests
cargo test

# Integration tests only
cargo test --test integration_tests

# With output
cargo test -- --nocapture

# Specific test
cargo test test_completions_endpoint
```

---

## 📊 Monitoring

### Prometheus Metrics

Access metrics at `http://localhost:3000/metrics`

**Key Metrics**:
- `chat_completions_requests_total`: Request count
- `chat_inference_duration_seconds`: Inference latency
- `chat_generated_tokens_total`: Token throughput
- `completions_errors_total`: Error rate

### Health Checks

```bash
# Liveness
curl http://localhost:3000/health

# Readiness (checks model availability)
curl http://localhost:3000/readiness
```

---

## 🔐 Security

### Enable Authentication

1. Edit `config.toml`:
```toml
[security]
enable_auth = true

[[security.api_keys]]
key = "sk-your-secret-key-here"
name = "production"
rate_limit_per_minute = 100
enabled = true
```

2. Include API key in requests:
```bash
curl -H "Authorization: Bearer sk-your-secret-key-here" \
  http://localhost:3000/completions
```

### Rate Limiting

- **Default**: 60 requests/minute per IP
- **Per-key**: Configurable in `config.toml`
- **Enforcement**: Automatic via middleware

---

## 🛠️ Development

### Project Structure

```
.
├── src/
│   ├── bin/
│   │   └── server.rs          # Entry point
│   ├── config.rs              # Configuration
│   ├── engine.rs              # Inference engine
│   ├── engine_mock.rs         # Test mock
│   ├── lib.rs                 # Library root
│   ├── models.rs              # Data models
│   ├── routes.rs              # HTTP handlers
│   └── state.rs               # Application state
├── tests/
│   ├── integration_tests.rs   # API tests
│   ├── config_tests.rs        # Config tests
│   └── middleware_tests.rs    # Middleware tests
├── public/
│   └── index.html             # Web UI
├── docs/
│   ├── API_REFERENCE.md       # API documentation
│   └── PROJECT_DOCUMENTATION.md # Complete guide
├── docker/
│   ├── Dockerfile             # CPU build
│   ├── Dockerfile.cuda        # GPU build
│   ├── docker-compose.yml     # Orchestration
│   ├── prometheus.yml         # Metrics config
│   └── README.md              # Docker guide
├── scripts/
│   ├── build_cuda_wsl.sh      # CUDA build script
│   ├── build_cpu_wsl.sh       # CPU build script
│   └── upgrade_cuda_wsl.sh    # CUDA upgrade script
├── Cargo.toml                 # Dependencies
├── config.example.toml        # Config template
└── postman_collection.json    # API tests
```

### Adding a New Model

1. Edit `config.toml`:
```toml
[[models.available_models]]
id = "llama"
name = "meta-llama/Llama-3.2-1B"
context_length = 8192
```

2. Restart the service
3. Model loads lazily on first request

### Building Features

**CUDA Support**:
```bash
cargo build --release --features cuda
```

**Metal Support (macOS)**:
```bash
cargo build --release --features metal
```

**Flash Attention**:
```bash
cargo build --release --features flash-attn
```

---

## 📚 Additional Resources

- **[API Reference](docs/API_REFERENCE.md)**: Complete endpoint documentation
- **[Project Documentation](docs/PROJECT_DOCUMENTATION.md)**: Full setup and deployment guide
- **[Docker Guide](docker/README.md)**: Container deployment instructions
- **[Postman Collection](postman_collection.json)**: Ready-to-use API tests
- **[mistral.rs](https://github.com/EricLBuehler/mistral.rs)**: Underlying inference engine
- **[Candle](https://github.com/huggingface/candle)**: ML framework

---

## 🤝 Contributing

Contributions welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Submit a pull request

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- **[mistral.rs](https://github.com/EricLBuehler/mistral.rs)**: High-performance Rust inference
- **[Candle](https://github.com/huggingface/candle)**: Minimalist ML framework
- **[Axum](https://github.com/tokio-rs/axum)**: Ergonomic web framework
- **Rust Community**: For amazing tooling and libraries

---

## 📧 Contact

For questions or issues, please open a GitHub issue or contact the maintainers.

**Happy inferencing! 🦀✨**
