# Rust LLM Inference Service

Complete documentation for the Rust LLM Inference Service - a production-ready, OpenAI-compatible LLM inference server with GPU acceleration and modern React frontend.

---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Getting Started](#getting-started)
- [Frontend Architecture](#frontend-architecture)
- [Backend Architecture](#backend-architecture)
- [Configuration](#configuration)
- [Deployment](#deployment)
- [Development](#development)
- [Testing](#testing)
- [Monitoring](#monitoring)
- [Troubleshooting](#troubleshooting)

---

## Overview

The Rust LLM Inference Service is a high-performance, production-ready server for running large language models locally with GPU acceleration. It features a React + TypeScript frontend and Rust backend, providing OpenAI-compatible APIs, real-time streaming, session management, and enterprise-grade security features.

**Key Highlights**:
- 🚀 **GPU Accelerated**: CUDA support for NVIDIA GPUs (10-50x faster than CPU)
- ⚛️ **Modern React UI**: TypeScript + Vite + Tailwind CSS + Zustand
- 🔄 **Real-time Streaming**: WebSocket streaming with live token generation
- 💬 **Session Management**: Multi-session support with persistent conversation history
- 🔒 **Enterprise Security**: API key authentication, rate limiting, content validation
- 📊 **Observability**: Prometheus metrics, health checks, structured logging
- 🐳 **Cloud Native**: Docker containers with GPU support
- 🎨 **Rich UI Features**: Markdown rendering, syntax highlighting, export history

---

## Features

### Frontend Capabilities

#### 1. Modern React Architecture
- **React 19**: Latest React with TypeScript
- **Vite**: Lightning-fast build tool with hot module replacement
- **Zustand**: Simple and scalable state management
- **Tailwind CSS v3**: Utility-first styling with custom animations
- **Component Structure**:
  - `App.tsx`: Root component with session lifecycle
  - `Sidebar.tsx`: Session list, settings, export (303 lines)
  - `ChatContainer.tsx`: Main chat UI with auto-scroll (120 lines)
  - `Message.tsx`: Markdown rendering with code highlighting (85 lines)

#### 2. Real-time Communication
- **WebSocket Hook**: Custom `useWebSocket` for streaming
- **Live Token Display**: Real-time tokens/second calculation
- **Auto-scroll**: Smart scrolling that preserves user position
- **Stop Generation**: Cancel in-progress responses
- **Connection Status**: Visual indicator for WebSocket state

#### 3. Rich Text Features
- **Markdown Rendering**: Full markdown support with `react-markdown`
- **Syntax Highlighting**: Code blocks with `rehype-highlight`
- **Copy Code Buttons**: One-click copy for code blocks
- **GitHub Dark Theme**: Professional code styling
- **Inline Code**: Styled inline code elements

#### 4. Session Management UI
- **Session List**: Sidebar showing all sessions (first 8 chars of UUID)
- **Active Highlighting**: Visual indicator for current session
- **Quick Switching**: Click to switch between sessions
- **Delete Sessions**: Remove with confirmation dialog
- **Export History**: Download chat as JSON with metadata
- **New Chat Button**: Create new sessions instantly

#### 5. Advanced Settings Panel
- **Model Selection**: Dropdown for available models
- **Device Selection**: Choose CPU or CUDA
- **Temperature Control**: Slider (0.1 - 2.0)
- **Top-P Sampling**: Nucleus sampling control
- **Top-K Sampling**: Integer input for top-k
- **Max Tokens**: Response length limit
- **Repeat Penalty**: Duplicate word control
- **System Prompt**: Custom system instructions
- **Reset Defaults**: One-click restore to defaults

### Backend Capabilities (Rust)

#### 1. High-Performance Inference
- **mistral.rs Engine**: Industry-leading Rust inference engine
- **GPU Acceleration**: CUDA 12.1+ support for NVIDIA GPUs
- **Model Pre-warming**: Both models load on server startup
- **CPU Fallback**: Automatic fallback when GPU unavailable
- **Metal Support**: macOS GPU acceleration (optional)
- **Lazy Loading**: Models cached after first load
- **Multiple Models**: Support for Qwen, Phi-3.5, and compatible GGUF models

#### 2. Web Service & APIs
- **REST API**: `/completions` and `/chat/completions` endpoints
- **WebSocket**: Real-time bidirectional streaming at `/chat/ws`
- **Static Files**: Serves frontend from `frontend/dist/`
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
- **Multi-Session**: Independent conversation threads with UUIDs
- **Persistent Storage**: History saved to SQLite (`sessions.db`)
- **Auto-Trimming**: Keep last 20 messages to prevent context overflow
- **History API**: Query and manage conversation history via REST
- **Session CRUD**: Create, read, update, delete operations
#### 4. Security & Governance
- **API Key Authentication**: Bearer token support
- **Rate Limiting**: Per-key or per-IP request throttling
- **Content Validation**: Prompt and response length limits
- **CORS Support**: Configurable cross-origin policies
- **Input Sanitization**: Protection against malicious inputs

#### 5. Observability
- **Prometheus Metrics**: Comprehensive performance tracking
- **Health Checks**: `/health` and `/readiness` endpoints
- **Structured Logging**: Configurable log levels
- **Key Metrics**:
  - Request counts
  - Inference latency
  - Token generation rate
  - Error rates
  - Session statistics

#### 6. Deployment
- **Docker Support**: Multi-stage builds for CPU and GPU
- **Docker Compose**: Integrated stack with Prometheus and Grafana
- **Health Probes**: Container-level health checking
- **Volume Mounts**: Persistent models and data
- **Environment Config**: Override settings via env vars

---

## Getting Started

### Prerequisites

- **Rust**: 1.75+ ([Install Rust](https://rustup.rs/))
- **Node.js**: 18+ and npm ([Install Node.js](https://nodejs.org/))
- **Git**: For cloning repository
- **(Optional)** NVIDIA GPU with CUDA 12.1+ for GPU acceleration
- **(Optional)** Docker for containerized deployment

### Quick Start

#### Option 1: Run from Source

```bash
# Clone repository
git clone https://github.com/KaichengXu007/1724-Rust-Project.git
cd 1724-Rust-Project

# Build frontend
cd frontend
npm install
npm run build
cd ..

# Run backend with GPU (recommended)
cargo run --release --features cuda --bin server

# OR run backend with CPU only
cargo run --release --bin server

# Open browser
# Navigate to http://localhost:3000
```

#### Option 2: Frontend Development Mode

For hot reload during frontend development:

```bash
# Terminal 1: Run backend
cargo run --release --features cuda --bin server

# Terminal 2: Run frontend dev server
cd frontend
npm run dev
# Frontend will be available at http://localhost:5173
```

#### Option 3: Docker

```bash
# Clone repository
git clone https://github.com/KaichengXu007/1724-Rust-Project.git
cd 1724-Rust-Project

# Run with GPU
docker-compose -f docker/docker-compose.yml up llm-gpu

# OR run with CPU
docker-compose -f docker/docker-compose.yml up llm-cpu

# Open browser
# Navigate to http://localhost:3000
```

The service will:
- Start on port 3000
- Serve the React frontend from `frontend/dist/`
- Pre-warm models on startup
- Create sessions automatically

### First Request

Test the API with cURL:
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

## Frontend Architecture

### Technology Stack

- **React 19**: Latest React with concurrent features
- **TypeScript 5.x**: Type-safe code
- **Vite 7.2**: Fast build tool with HMR
- **Zustand 5.0**: Lightweight state management
- **Tailwind CSS 3**: Utility-first styling
- **react-markdown 10.1**: Markdown rendering
- **rehype-highlight 7.0**: Syntax highlighting

### Component Hierarchy

```
App.tsx (Root Component)
├── Sidebar.tsx
│   ├── Header with Logo
│   ├── New Chat Button
│   ├── Export History Button
│   ├── Session List
│   │   └── SessionItem (with delete button)
│   └── Settings Panel
│       ├── Model Selection
│       ├── Device Selection
│       ├── Temperature Slider
│       ├── Top-P Input
│       ├── Top-K Input
│       ├── Max Tokens Input
│       ├── Repeat Penalty Input
│       ├── System Prompt Textarea
│       └── Reset Button
└── ChatContainer.tsx
    ├── Welcome Screen (when no messages)
    ├── Message List
    │   └── Message.tsx (for each message)
    │       ├── ReactMarkdown
    │       ├── Code Blocks with Copy Button
    │       └── Typing Indicator
    ├── Stop Generation Button (when generating)
    └── Input Area
        ├── Auto-resizing Textarea
        └── Send Button
```

### State Management (Zustand)

**Store Structure** (`chatStore.ts`):
```typescript
{
  // Session
  sessionId: string,
  sessions: string[],
  messages: Message[],
  
  // Generation
  isGenerating: boolean,
  isConnected: boolean,
  tokenCount: number,
  tokensPerSecond: number,
  
  // Settings
  settings: ChatSettings,
  
  // Actions (12 total)
  setSessionId, setMessages, addMessage,
  updateLastMessage, setSessions, addSession,
  removeSession, updateSettings, resetSettings,
  setIsGenerating, setIsConnected, setTokenCount,
  setTokensPerSecond, clearMessages
}
```

### API Service Layer

**`api.ts`** provides:
- `getSessions()`: Fetch all session IDs
- `deleteSession(id)`: Delete a session
- `getHistory(id)`: Load conversation history
- `rollbackHistory(id, amount)`: Remove last N messages
- `createWebSocket()`: Create WebSocket connection

### WebSocket Hook

**`useWebSocket.ts`** handles:
- Connection management with auto-reconnect
- Message sending with settings
- Token streaming
- Tokens/second calculation
- Stop generation
- Connection status updates

### Key Features

1. **Auto-scroll**: Scrolls to bottom on new messages, preserves position when user scrolls up
2. **Code Highlighting**: GitHub Dark theme with copy buttons on hover
3. **Session Persistence**: Sessions stored in backend, synced with UI
4. **Export History**: Downloads JSON with messages, settings, and metadata
5. **Responsive Settings**: All changes update in real-time
6. **Error Handling**: Graceful fallbacks for API failures

---

## Backend Architecture

### System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Client Layer                         │
│       (React Frontend, cURL, Python, JavaScript, etc.)      │
└────────────────────────┬────────────────────────────────────┘
                         │ HTTP/WebSocket
┌────────────────────────▼────────────────────────────────────┐
│                    Axum Web Server                          │
│              Static Files • Routes • Middleware              │
├─────────────────────────────────────────────────────────────┤
│  GET /              → frontend/dist/index.html              │
│  GET /sessions      → List all session IDs                  │
│  POST /completions  → Generate completion                   │
│  WS /chat/ws        → Real-time streaming                   │
│  DELETE /chat/history/:id → Delete session                  │
├─────────────────────────────────────────────────────────────┤
│                Application State (Arc<RwLock>)              │
│         Session Manager • Model Cache • Rate Limiters        │
├─────────────────────────────────────────────────────────────┤
│              M1 Engine Adapter (mistral.rs)                 │
│         Model Loader • Tokenization • Sampling              │
├─────────────────────────────────────────────────────────────┤
│                   Candle ML Framework                       │
│              CUDA • Metal • CPU • Quantization              │
└─────────────────────────────────────────────────────────────┘
```
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
docker build -t llm-service:cpu -f docker/Dockerfile .

# GPU version
docker build -t llm-service:gpu -f docker/Dockerfile.cuda .
```

**Run Containers**:
```bash
# CPU
docker run -p 3000:3000 \
  -v $(pwd)/models:/app/models \
  -v $(pwd)/sessions.db:/app/sessions.db \
  llm-service:cpu

# GPU
docker run --gpus all -p 3000:3000 \
  -v $(pwd)/models:/app/models \
  -v $(pwd)/sessions.db:/app/sessions.db \
  llm-service:gpu
```

### Docker Compose Stack

```bash
# Start full stack (app + Prometheus + Grafana)
docker-compose -f docker/docker-compose.yml up -d

# View logs
docker-compose -f docker/docker-compose.yml logs -f llm-gpu

# Stop stack
docker-compose -f docker/docker-compose.yml down
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
├── docker/
│   ├── Dockerfile             # CPU image
│   ├── Dockerfile.cuda        # GPU image
│   ├── docker-compose.yml     # Docker stack
│   ├── prometheus.yml         # Metrics config
│   ├── .dockerignore          # Build optimization
│   └── README.md              # Docker guide
├── scripts/
│   ├── build_cuda_wsl.sh      # CUDA build script
│   ├── build_cpu_wsl.sh       # CPU build script
│   └── upgrade_cuda_wsl.sh    # CUDA upgrade script
├── Cargo.toml                 # Dependencies
├── config.example.toml        # Example config
├── postman_collection.json    # API tests
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
