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
- 💬 **Session Management**: Multi-session support with SQLite-backed persistent storage
- 🔒 **Enterprise Security**: API key authentication, rate limiting, content validation
- 📊 **Observability**: Prometheus metrics, health checks, structured logging
- 🎨 **Rich UI Features**: Markdown rendering, syntax highlighting, export history
- 🗄️ **Data Persistence**: SQLite database for session and conversation history

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

**WebSocket Protocol**:
```javascript
// Connect with session ID in query params
const ws = new WebSocket('ws://localhost:3000/chat/ws?session_id=my-session');

// Send request (JSON)
ws.send(JSON.stringify({
  model_name: "qwen",
  prompt: "Your question here",
  temperature: 0.7,
  max_token: 256,
  device: "cuda"
}));

// Receive tokens (one per message)
ws.onmessage = (event) => {
  console.log('Token:', event.data); // Plain text, not JSON
};
```

#### 3. Session Management
- **Multi-Session**: Independent conversation threads with UUIDs
- **Persistent Storage**: History saved to SQLite (`sessions.db`)
- **Auto-Trimming**: Keep last 20 messages to prevent context overflow
- **History API**: Query and manage conversation history via REST
- **Session CRUD**: Create, read, update, delete operations
- **Rollback Support**: Remove last N messages from conversation

**Session API Endpoints**:
- `GET /sessions` - List all session IDs
- `GET /chat/history/:session_id` - Get conversation history
- `DELETE /chat/history/:session_id` - Delete session
- `POST /chat/history/:session_id/rollback` - Rollback N messages (body: `{"amount": 2}`)
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
host = "127.0.0.1"
port = 3000
log_level = "info"  # trace, debug, info, warn, error

[models]
default_device = "cuda"  # cuda, cpu, metal
max_concurrent_requests = 10

# Available models configuration
[[models.available_models]]
id = "qwen"
name = "Qwen/Qwen2.5-0.5B-Instruct"
context_length = 4096
# path = "/path/to/local/model"  # Optional: local model path
# quantization = "q4"  # Optional: q4, q8, bf16

[[models.available_models]]
id = "phi"
name = "microsoft/Phi-3.5-mini-instruct"
context_length = 4096

[security]
enable_auth = false  # Set to true to require API keys
allowed_origins = ["*"]  # CORS configuration

# API Keys - only used if enable_auth = true
# [[security.api_keys]]
# key = "sk-your-secret-key-here"
# name = "default"
# rate_limit_per_minute = 100
# enabled = true

[limits]
max_prompt_length = 8192
max_response_tokens = 2048
max_sessions = 1000
session_ttl_seconds = 3600  # 1 hour
default_rate_limit_per_minute = 60

[observability]
enable_metrics = true
enable_tracing = true
metrics_path = "/metrics"
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
│  POST /chat/history/:id/rollback → Rollback messages        │
├─────────────────────────────────────────────────────────────┤
│         Application State (Arc + Mutex + SQLite)            │
│  Session Manager • Model Cache • Rate Limiters • SQLite DB   │
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
│  │    • SessionStore (SQLite + In-memory cache)         │  │
│  │    • RateLimiter (DashMap per-key tracking)          │  │
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

### Production Deployment

**Build for Production**:
```bash
# Build frontend
cd frontend
npm install
npm run build
cd ..

# Build backend with optimizations
cargo build --release --features cuda

# Binary will be at: target/release/server
```

**Run as Service (Linux with systemd)**:

Create `/etc/systemd/system/llm-inference.service`:
```ini
[Unit]
Description=LLM Inference Service
After=network.target

[Service]
Type=simple
User=llm
WorkingDirectory=/opt/llm-inference
ExecStart=/opt/llm-inference/target/release/server
Restart=always
RestartSec=10
Environment="RUST_LOG=info"
Environment="CUDA_VISIBLE_DEVICES=0"

[Install]
WantedBy=multi-user.target
```

**Data Persistence**:
- Sessions: `sessions.db` (SQLite database)
- Models: Cached in `~/.cache/huggingface/` by default
- Logs: Captured by systemd journal (`journalctl -u llm-inference -f`)

## Development

### Project Structure

```
1724-Rust-Project/
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
│   │   ├── assets/             # Static assets
│   │   ├── App.tsx             # Root component
│   │   ├── App.css             # Component styles
│   │   ├── index.css           # Global Tailwind styles
│   │   └── main.tsx            # React entry point
│   ├── public/
│   │   └── index.html          # HTML template
│   ├── dist/                   # Production build (generated)
│   ├── package.json            # NPM dependencies
│   ├── tsconfig.json           # TypeScript config
│   ├── tailwind.config.js      # Tailwind CSS config
│   └── vite.config.ts          # Vite build config
├── src/                        # Rust backend
│   ├── bin/
│   │   └── server.rs           # Entry point with pre-warming
│   ├── config.rs               # TOML configuration loader
│   ├── engine.rs               # Inference engine adapter
│   ├── engine_mock.rs          # Mock engine for testing
│   ├── lib.rs                  # Library root
│   ├── middleware.rs           # Rate limiter implementation
│   ├── models.rs               # Request/response data models
│   ├── routes.rs               # HTTP/WebSocket handlers
│   └── state.rs                # App state & SQLite session store
├── tests/
│   ├── integration_tests.rs    # API integration tests
│   ├── config_tests.rs         # Config validation tests
│   └── middleware_tests.rs     # Rate limiter tests
├── docs/
│   ├── API_REFERENCE.md        # API documentation
│   ├── PROJECT_DOCUMENTATION.md # Complete technical guide
│   └── PROJECT_PROPOSAL.md     # Original project proposal
├── scripts/
│   └── test-rate-limit.ps1     # Rate limiting test script
├── Cargo.toml                  # Rust dependencies & features
├── config.toml                 # Runtime configuration
├── config.example.toml         # Configuration template
├── sessions.db                 # SQLite session database
├── postman_collection.json     # Postman API tests
├── CODE_ANALYSIS.md            # Comprehensive code analysis
└── README.md                   # Quick start guide
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

### Rate Limit Testing (PowerShell)

To quickly validate the server-side rate limiting behavior from a Windows environment, a reusable PowerShell script is included at `scripts/test-rate-limit.ps1`.

How it works:
- The script issues repeated HTTP GET requests to the target URL (default `http://127.0.0.1:3000/sessions`).
- It prints per-request status and, on 429 responses, prints the response body and all response headers (including `X-RateLimit-Limit`, `X-RateLimit-Remaining`, and `X-RateLimit-Reset`).

Run the script (from the project root):
```powershell
cd "<project-root>"
.\scripts\test-rate-limit.ps1
```

Examples with options:
```powershell
# target a different URL
.\scripts\test-rate-limit.ps1 -Url 'http://127.0.0.1:3000/sessions' -Count 100 -DelayMs 50

# quick smoke test with default args
.\scripts\test-rate-limit.ps1
```

Interpreting output:
- `ok N (200)` — request succeeded; the script will also print any `X-RateLimit-*` headers returned by the server.
- `error N (429): {"error":"rate limit exceeded"}` — server rejected the request due to rate limiting; the following lines will list all response headers, including `X-RateLimit-Limit`, `X-RateLimit-Remaining` (usually `0`), and `X-RateLimit-Reset` (unix timestamp when window resets).

Notes and tips:
- If the default limit (60 requests/min) is too high for quick manual testing, temporarily lower `default_rate_limit_per_minute` in `config.toml` (for example to `3`), then restart the server.
- WebSocket connections cannot expose upgrade response headers to browser JS; to test WS behavior use the frontend UI (which will show an alert on immediate WS close) or trigger many WS upgrades from the browser Console.
- The script is intentionally conservative: it sleeps `DelayMs` between requests to avoid accidentally DoSing a real deployment; adjust as needed for local testing only.


---

## Monitoring

### Prometheus Metrics

Access metrics at `http://localhost:3000/metrics`

**Available Metrics**:
- `chat_completions_requests_total`: Chat completion request count
- `chat_inference_duration_seconds`: Inference latency histogram
- `chat_generated_tokens_total`: Total tokens generated in chat
- `completions_errors_total`: Completion errors
- `chat_completions_errors_total`: Chat completion errors
- `rate_limit_allowed_total`: Requests allowed by rate limiter
- `rate_limit_blocked_total`: Requests blocked by rate limiter
- `health_check_requests_total`: Health check endpoint calls
- `readiness_check_requests_total`: Readiness check endpoint calls
- `models_list_requests_total`: Model list requests
- `model_info_requests_total`: Model info requests
- `history_requests_total`: Session history requests

### Grafana Dashboards

To set up monitoring with Grafana:

1. Install Prometheus and Grafana on your system
2. Configure Prometheus to scrape `http://localhost:3000/metrics`
3. Add Prometheus as datasource in Grafana
4. Create custom dashboards or import community templates

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

#### 6. Database Locked
**Symptom**: `database is locked` or SQLite errors

**Solution**:
```bash
# Stop all running instances
# Windows PowerShell
Get-Process | Where-Object {$_.ProcessName -like "*server*"} | Stop-Process

# Linux/macOS
pkill -f server

# If corrupted, remove lock files
rm sessions.db-wal sessions.db-shm
```

#### 7. Frontend Build Errors
**Symptom**: `npm run build` fails

**Solution**:
```bash
cd frontend
rm -rf node_modules package-lock.json
npm cache clean --force
npm install
npm run build
```

#### 8. Session Not Persisting
**Symptom**: Sessions lost after restart

**Solution**:
- Check `sessions.db` exists in working directory
- Verify SQLite permissions (read/write)
- Check disk space
- Review logs for database errors

#### 9. WebSocket Connection Failed
**Symptom**: Chat doesn't stream, connection errors

**Solution**:
- Check backend is running (`http://localhost:3000/health`)
- Verify port 3000 is accessible
- Check browser console for errors
- Ensure no proxy/firewall blocking WebSocket
- Try refreshing the page

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
