# 🚀 Rust LLM Inference Service

**A high-performance, production-ready Large Language Model (LLM) inference service built with Rust backend and React frontend. Provides OpenAI-compatible APIs with real-time token streaming, session management, and a modern web UI.**

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/react-19.2.0-blue.svg)](https://reactjs.org/)
[![TypeScript](https://img.shields.io/badge/typescript-5.x-blue.svg)](https://www.typescriptlang.org/)

---

## Video Slide Presentation
[Click here]()

---

## Video Demo
[Click here]()

---

## Team Members
- **Zhiwen Yang** 1002422853 zhiwen.yang@mail.utoronto.ca
- **Ruijie Yao** 1010797853 chris.yao@mail.utoronto.ca
- **Kaicheng Xu** 1005680452 kaicheng.xu@mail.utoronto.ca

---

## 💡 Motivation

The landscape of LLM inference is dominated by Python-based solutions, which often struggle with performance, memory efficiency, and deployment complexity. We wanted to explore whether Rust—with its zero-cost abstractions, memory safety guarantees, and exceptional performance—could provide a superior foundation for production LLM serving.

This project fills a critical gap in the Rust ecosystem by providing:
- **Production-grade LLM serving** with session persistence and stateful conversations
- **True streaming inference** with WebSocket support for real-time token delivery
- **Enterprise-ready features** like rate limiting, metrics, and health checks
- **Modern web UI** that rivals commercial LLM interfaces

Our motivation was to prove that Rust can compete with established Python frameworks while offering better resource utilization, faster inference, and more reliable deployments—making LLMs accessible even on resource-constrained environments.

---

## 🎯 Objectives

This project aims to achieve the following objectives:

1. **High-Performance Inference**: Leverage Rust's performance characteristics and Candle ML framework to deliver low-latency LLM inference with efficient memory usage

2. **Production-Ready Architecture**: Build a complete inference service with session management, persistence, rate limiting, and observability—not just a proof of concept

3. **Developer Experience**: Provide OpenAI-compatible APIs and comprehensive documentation to enable easy integration and deployment

4. **Modern UI/UX**: Create an intuitive React-based interface with real-time streaming, markdown rendering, and session management that matches commercial LLM platforms

5. **Deployment Flexibility**: Support multiple hardware configurations (CPU, CUDA, Metal) and provide a single-binary deployment with minimal dependencies

6. **Ecosystem Contribution**: Demonstrate Rust's viability for ML workloads and contribute to the growing Rust ML ecosystem through practical, documented examples

---

## ✨ Features

### 🎯 Core Capabilities
- **Multiple Model Support**: Load and manage multiple Huggingface-format models via Candle and Mistral.rs
- **Streaming Inference**: Real-time token streaming via WebSocket with tokens/second display
- **Model Pre-warming**: Automatic model loading at startup for zero-latency first requests
- **Session Management**: Stateful conversations with full history and session switching
  - SQLite-backed persistence with per-session durability
  - Automatic context pruning (maintains last 20 messages)
  - Session rollback support for conversation editing
- **Modern React UI**: 
  - Built with React 19 + TypeScript + Vite
  - Zustand state management
  - Tailwind CSS v3 for styling
  - Real-time markdown rendering with syntax highlighting
  - Code copy buttons and dark mode
  - Session history with export functionality
  - Advanced model settings panel

### 🔒 Security & Governance
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
│   Vite • Zustand • Tailwind CSS • React Markdown            │
│   WebSocket • Code Highlighting • Session Management        │
└────────────────────────┬────────────────────────────────────┘
                         │ HTTP/WS
┌────────────────────────▼────────────────────────────────────┐
│                    Axum Web Framework                       │
│            Routes • Middleware • State Management           │
├─────────────────────────────────────────────────────────────┤
│  Authentication  │  Rate Limiting  │  Content Validation    │
├─────────────────────────────────────────────────────────────┤
│        Session Manager (HashMap + Mutex + SQLite)           │
│   In-memory Cache • Persistent Storage • Context Pruning    │
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

## 🚀 Quick Start (User's Guide)

### Prerequisites
- **Rust** 1.75+ (`rustup` recommended)
- **Node.js** 18+ and npm (for frontend development)
- **(Optional)** NVIDIA GPU + CUDA Toolkit 12.1  (for GPU acceleration)

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

### Available Endpoints

- `GET /models` - List all available models
- `GET /models/:model_id` - Get specific model information
- `GET /sessions` - List all session IDs
- `POST /completions` - Generate text completion
- `POST /chat/completions` - Chat completion (with streaming)
- `GET /chat/ws` - WebSocket endpoint for real-time streaming
- `GET /chat/history/:session_id` - Get session conversation history
- `DELETE /chat/history/:session_id` - Delete a session
- `POST /chat/history/:session_id/rollback` - Rollback N messages from history
- `GET /health` - Health check endpoint
- `GET /readiness` - Readiness check (validates model availability)
- `GET /metrics` - Prometheus metrics

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
    "model_name": "qwen",
    "prompt": "What is async/await?",
    "max_token": 256,
    "temperature": 0.7,
    "device": "cuda"
  }'
```

**WebSocket chat** (real-time streaming):
```javascript
const ws = new WebSocket('ws://localhost:3000/chat/ws?session_id=my-session');

ws.onopen = () => {
  ws.send(JSON.stringify({
    model_name: "qwen",
    prompt: "Explain Rust ownership",
    temperature: 0.7,
    max_token: 256,
    device: "cuda"
  }));
};

ws.onmessage = (event) => {
  console.log('Token:', event.data); // Receives tokens one by one
};
```

---

## 🧪 Testing

Run the test suite:

```bash
# All tests
cargo test

# Specific test suites
cargo test --test integration_tests  # Integration tests
cargo test --test config_tests       # Config validation tests
cargo test --test middleware_tests   # Middleware tests
cargo test --lib                     # Unit tests only

# With verbose output
RUST_LOG=debug cargo test -- --nocapture
```

### Frontend Tests

```bash
cd frontend
npm run test  # (when implemented)
```

---

## 📊 Monitoring

### Prometheus Metrics

Access metrics at `http://localhost:3000/metrics`

**Key Metrics**:
- `chat_completions_requests_total`: Chat completion request count
- `chat_inference_duration_seconds`: Inference latency histogram
- `chat_generated_tokens_total`: Total tokens generated
- `completions_errors_total`: Completion errors
- `chat_completions_errors_total`: Chat completion errors
- `rate_limit_allowed_total`: Requests allowed by rate limiter
- `rate_limit_blocked_total`: Requests blocked by rate limiter
- `health_check_requests_total`: Health check endpoint calls
- `readiness_check_requests_total`: Readiness check endpoint calls
- `models_list_requests_total`: Model list requests
- `model_info_requests_total`: Model info requests
- `history_requests_total`: Session history requests

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

## 🔧 Troubleshooting

### Model Download Issues

Models are automatically downloaded from HuggingFace. If downloads fail:

1. **For gated models**, set your HuggingFace token:
```bash
export HF_TOKEN="your_token_here"
```

2. **For network issues**, download manually:
```bash
git lfs install
git clone https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct
```

3. **Use local path** in `config.toml`:
```toml
[[models.available_models]]
id = "qwen"
path = "./Qwen2.5-0.5B-Instruct"
```

### CUDA Build Errors

If `cargo build --features cuda` fails:

1. **Verify CUDA installation**:
```bash
nvcc --version
nvidia-smi
```

2. **Install CUDA Toolkit 12.1+**: [CUDA Downloads](https://developer.nvidia.com/cuda-downloads)

3. **Set environment variables** (Linux/macOS):
```bash
export CUDA_HOME=/usr/local/cuda
export PATH=$CUDA_HOME/bin:$PATH
export LD_LIBRARY_PATH=$CUDA_HOME/lib64:$LD_LIBRARY_PATH
```

### Port Conflicts

If port 3000 is in use, change it in `config.toml`:
```toml
[server]
port = 8080
```

### Database Locked Errors

1. Stop all running instances:
```bash
# Linux/macOS
pkill -f server

# Windows PowerShell
Get-Process | Where-Object {$_.ProcessName -like "*server*"} | Stop-Process
```

2. If corrupted, remove lock files:
```bash
rm sessions.db-wal sessions.db-shm
```

---

## 🛠️ Development

### Project Structure

```
.
├── frontend/                       # React frontend
│   ├── src/
│   │   ├── components/
│   │   │   ├── ChatContainer.tsx   # Main chat UI
│   │   │   ├── Message.tsx         # Message rendering
│   │   │   └── Sidebar.tsx         # Session & settings
│   │   ├── hooks/
│   │   │   └── useWebSocket.ts     # WebSocket streaming
│   │   ├── services/
│   │   │   └── api.ts              # API client
│   │   ├── store/
│   │   │   └── chatStore.ts        # Zustand state
│   │   ├── assets/                 # Static assets
│   │   ├── App.tsx                 # Root component
│   │   ├── App.css                 # Component styles
│   │   ├── index.css               # Global Tailwind styles
│   │   └── main.tsx                # React entry point
│   ├── public/                     # Public assets
│   │   └── index.html              # HTML template
│   ├── dist/                       # Production build (generated)
│   ├── package.json                # NPM dependencies
│   ├── tsconfig.json               # TypeScript config
│   ├── tailwind.config.js          # Tailwind CSS config
│   └── vite.config.ts              # Vite build config
├── src/                            # Rust backend
│   ├── bin/
│   │   └── server.rs               # Entry point with pre-warming
│   ├── config.rs                   # TOML configuration loader
│   ├── engine.rs                   # Inference engine adapter
│   ├── engine_mock.rs              # Mock engine for testing
│   ├── lib.rs                      # Library root
│   ├── middleware.rs               # Rate limiter implementation
│   ├── models.rs                   # Request/response data models
│   ├── routes.rs                   # HTTP/WebSocket handlers
│   └── state.rs                    # App state & SQLite session store
├── tests/
│   ├── integration_tests.rs        # API integration tests
│   ├── config_tests.rs             # Config validation tests
│   └── middleware_tests.rs         # Rate limiter tests
├── docs/
│   ├── API_REFERENCE.md            # API documentation
│   ├── PROJECT_DOCUMENTATION.md    # Complete technical guide
│   └── PROJECT_PROPOSAL.md         # Original project proposal
├── scripts/
│   └── test-rate-limit.ps1         # Rate limiting test script
├── Cargo.toml                      # Rust dependencies & features
├── config.toml                     # Runtime configuration
├── config.example.toml             # Configuration template
├── sessions.db                     # SQLite session database
└── postman_collection.json         # Postman API tests
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

---
## Contributions

- **Ruijie Yao (Backend / Infrastructure)**
    - Bootstrapped the Rust inference core and mistral.rs integration, then iterated on model loading and performance tuning.
    - Implemented production-grade persistence with the SQLite-backed session store and graceful deletion safeguards, plus improved engine resilience and canonical model resolution for pre-warming.
    - Coordinated backend branches and reconciled parallel workstreams to keep releases stable.

- **Kaicheng Xu (Systems / Platform)**
    - Delivered CUDA enablement, observability stack, and the initial session management pipeline that underpins the core features described above.
    - Added rate-limiting middleware, expanded automated tests, mock inference helpers, and stream APIs to harden the service surface.

- **Zhiwen Yang (Frontend / Documentation)**
  - Reorganized the monorepo layout and iterated on the React UI/UX, including the modern sidebar, export workflow, and logging guidance.
  - Led the README/doc refresh cycles, ensuring the Quick Start, deployment, and configuration guidance reflect the current feature set.
  - Maintained day-to-day polish on structure and wording, keeping the documentation aligned with ongoing work.

---

## Lessons Learned & Conclusion

- **Rust for production inference**: Building a full-stack LLM service in Rust was viable but required careful ownership of async runtimes, memory usage, and third-party crates. In return, we gained predictable performance, strong typing across API layers, and simpler deployment once the core was stabilized.
- **Observability early pays off**: Instrumentation, health probes, and rate limiting surfaced panic loops, runaway sessions, and frontend regressions before users felt them. Shipping with observability from day one reduced debugging overhead later.
- **Tight frontend/backend feedback loop**: Keeping the React UI and Axum backend in sync via OpenAI-compatible contracts allowed both halves to iterate quickly without blocking each other, revealing API ergonomics issues early.
- **Persistence evolution**: Moving from JSON files to SQLite persistence unlocked durability, editing, and rollback while remaining lightweight enough for both local and production use.

---

## 📚 Additional Resources

- **[API Reference](docs/API_REFERENCE.md)**: Complete endpoint documentation
- **[Project Documentation](docs/PROJECT_DOCUMENTATION.md)**: Full setup and deployment guide
- **[Postman Collection](postman_collection.json)**: Ready-to-use API tests
- **[mistral.rs](https://github.com/EricLBuehler/mistral.rs)**: Underlying inference engine
- **[Candle](https://github.com/huggingface/candle)**: ML framework

---

## ❓ FAQ

**Q: Can I use multiple GPUs?**  
A: Currently, the service uses a single GPU. Set `CUDA_VISIBLE_DEVICES=0` to select which GPU to use.

**Q: How much RAM/VRAM do I need?**  
A: Depends on model size:
- Qwen2.5-0.5B: ~2GB VRAM, 4GB RAM
- Phi-3.5-mini: ~4GB VRAM, 8GB RAM
- Larger models: Scale accordingly

**Q: Can I use quantized models?**  
A: Yes, configure in `config.toml`:
```toml
[[models.available_models]]
id = "qwen-q4"
name = "Qwen/Qwen2.5-0.5B-Instruct"
quantization = "q4"
```

**Q: Do sessions expire?**  
A: Sessions persist in SQLite. TTL is configured but not currently enforced. Sessions remain until explicitly deleted.

**Q: Can I run without a GPU?**  
A: Yes, omit the `--features cuda` flag:
```bash
cargo run --release --bin server
```
Models will run on CPU (slower performance).

**Q: How do I backup sessions?**  
A: Copy the SQLite database:
```bash
cp sessions.db sessions.db.backup
```

**Q: What's the difference between `/completions` and `/chat/completions`?**  
A: `/completions` is for raw text completion. `/chat/completions` supports conversation history and session management.

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
