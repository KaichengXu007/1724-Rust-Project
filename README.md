# 🚀 Rust LLM Inference Service

A high-performance, production-ready Large Language Model (LLM) inference service built with Rust backend and React frontend. Provides OpenAI-compatible APIs with real-time token streaming, session management, and a modern web UI.

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/react-19.2.0-blue.svg)](https://reactjs.org/)
[![TypeScript](https://img.shields.io/badge/typescript-5.x-blue.svg)](https://www.typescriptlang.org/)
[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](docker/)

## ✨ Features

### 🎯 Core Capabilities
- **Multiple Model Support**: Load and manage multiple Huggingface-format models via Candle and Mistral.rs
- **Streaming Inference**: Real-time token streaming via WebSocket with tokens/second display
- **Session Management**: Stateful conversations with full history and session switching
  - SQLite-backed persistence with per-session durability
- **Modern React UI**: 
  - Built with React 19 + TypeScript + Vite
  - Zustand state management
  - Tailwind CSS v3 for styling
  - Real-time markdown rendering with syntax highlighting
  - Code copy buttons and dark mode
  - Session history with export functionality
  - Advanced model settings panel

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
│                React Frontend (TypeScript)                  │
│   Vite • Zustand • Tailwind CSS • React Markdown           │
│   WebSocket • Code Highlighting • Session Management        │
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
- **Frontend**:
  - **`App.tsx`**: Root component with session lifecycle management
  - **`Sidebar.tsx`**: Session list, settings panel, export functionality
  - **`ChatContainer.tsx`**: Main chat interface with auto-scroll
  - **`Message.tsx`**: Markdown rendering with syntax highlighting
  - **`chatStore.ts`**: Zustand state management (12 actions)
  - **`api.ts`**: API service layer with WebSocket support
  - **`useWebSocket.ts`**: WebSocket hook for streaming
- **Backend**:
  - **`routes.rs`**: HTTP endpoints, WebSocket handlers
  - **`engine.rs`**: Inference abstraction, model management
  - **`state.rs`**: Application state, session persistence
  - **`server.rs`**: Entry point with model pre-warming

---

## 🚀 Quick Start

### Prerequisites
- **Rust** 1.75+ (`rustup` recommended)
- **Node.js** 18+ and npm (for frontend development)
- **(Optional)** NVIDIA GPU + CUDA Toolkit 12.1+ for GPU acceleration
- **(Optional)** Docker for containerized deployment

### Installation

1. **Clone the repository**:
```bash
git clone https://github.com/KaichengXu007/1724-Rust-Project.git
cd 1724-Rust-Project
```

2. **Build the frontend**:
```bash
cd frontend
npm install
npm run build
cd ..
```

3. **Create configuration** (optional):
```bash
cp config.example.toml config.toml
# Edit config.toml to customize settings
```

4. **Run the service**:

**CPU Mode**:
```bash
cargo run --release --bin server
```

**GPU Mode (CUDA)**:
```bash
cargo run --release --features cuda --bin server
```

5. **Access the web UI**:
Open your browser to `http://localhost:3000`

### Frontend Development

To run the frontend in development mode with hot reload:

```bash
cd frontend
npm run dev
```

Then run the backend server separately. The Vite dev server will proxy API requests to the backend.

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

---

## 🧪 Testing

Run the test suite:

```bash
# All tests
cargo test
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

### Rate Limiting

- **Default**: 60 requests/minute per IP
- **Per-key**: Configurable in `config.toml`
- **Enforcement**: Automatic via middleware

---

## 🛠️ Development

### Project Structure

```
.
├── frontend/                   # React frontend
│   ├── src/
│   │   ├── components/
│   │   │   ├── ChatContainer.tsx  # Main chat UI
│   │   │   ├── Message.tsx        # Message rendering
│   │   │   └── Sidebar.tsx        # Session & settings
│   │   ├── hooks/
│   │   │   └── useWebSocket.ts    # WebSocket streaming
│   │   ├── services/
│   │   │   └── api.ts             # API client
│   │   ├── store/
│   │   │   └── chatStore.ts       # Zustand state
│   │   ├── App.tsx                # Root component
│   │   └── index.css              # Tailwind styles
│   ├── dist/                   # Production build
│   ├── package.json            # Dependencies
│   ├── tsconfig.json           # TypeScript config
│   ├── tailwind.config.js      # Tailwind config
│   └── vite.config.ts          # Vite config
├── src/                        # Rust backend
│   ├── bin/
│   │   └── server.rs           # Entry point
│   ├── config.rs               # Configuration
│   ├── engine.rs               # Inference engine
│   ├── engine_mock.rs          # Test mock
│   ├── lib.rs                  # Library root
│   ├── models.rs               # Data models
│   ├── routes.rs               # HTTP handlers
│   └── state.rs                # Application state
├── tests/
│   ├── integration_tests.rs    # API tests
│   ├── config_tests.rs         # Config tests
│   └── middleware_tests.rs     # Middleware tests
├── docs/
│   ├── API_REFERENCE.md        # API documentation
│   └── PROJECT_DOCUMENTATION.md # Complete guide
├── docker/
│   ├── Dockerfile              # CPU build
│   ├── Dockerfile.cuda         # GPU build
│   ├── docker-compose.yml      # Orchestration
│   ├── prometheus.yml          # Metrics config
│   └── README.md               # Docker guide
├── Cargo.toml                  # Rust dependencies
├── config.example.toml         # Config template
└── postman_collection.json     # API tests
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

## 🙏 Acknowledgments

- **[mistral.rs](https://github.com/EricLBuehler/mistral.rs)**: High-performance Rust inference
- **[Candle](https://github.com/huggingface/candle)**: Minimalist ML framework
- **[Axum](https://github.com/tokio-rs/axum)**: Ergonomic web framework
- **[React](https://reactjs.org/)**: UI library for building interactive interfaces
- **[Vite](https://vitejs.dev/)**: Next-generation frontend tooling
- **[Zustand](https://github.com/pmndrs/zustand)**: Simple state management
- **[Tailwind CSS](https://tailwindcss.com/)**: Utility-first CSS framework
- **Rust & TypeScript Communities**: For amazing tooling and libraries

---
