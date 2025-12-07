# Getting Started with Rust LLM Inference Service

This guide will help you get the Rust LLM Inference Service up and running in minutes.

## Prerequisites

- **Rust**: Version 1.75 or later ([Install Rust](https://rustup.rs/))
- **Git**: For cloning the repository
- **(Optional)** NVIDIA GPU with CUDA 12.2+ for GPU acceleration
- **(Optional)** Docker for containerized deployment

## Installation

### Option 1: Run from Source (Recommended for Development)

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/rust-llm-inference.git
   cd rust-llm-inference
   ```

2. **Run with default settings**:
   ```bash
   # CPU mode
   cargo run --release --bin server
   
   # OR GPU mode (if you have CUDA)
   cargo run --release --features cuda --bin server
   ```

3. **Open your browser**:
   Navigate to `http://localhost:3000`

That's it! The service will:
- Start on port 3000
- Load models on first request
- Use sensible defaults from code

### Option 2: Docker (Recommended for Production)

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/rust-llm-inference.git
   cd rust-llm-inference
   ```

2. **Run with Docker Compose**:
   ```bash
   # CPU version
   docker-compose up llm-cpu
   
   # OR GPU version (requires nvidia-docker)
   docker-compose up llm-gpu
   ```

3. **Access the service**:
   - Web UI: `http://localhost:3000`
   - API: `http://localhost:3000/models`

## First Steps

### 1. Check Service Health

```bash
curl http://localhost:3000/health
```

Expected response:
```json
{
  "status": "ok",
  "uptime": "running",
  "timestamp": "2025-12-07T..."
}
```

### 2. List Available Models

```bash
curl http://localhost:3000/models
```

Expected response:
```json
{
  "models": [
    "Qwen/Qwen2.5-0.5B-Instruct",
    "microsoft/Phi-3.5-mini-instruct"
  ]
}
```

### 3. Generate Your First Completion

```bash
curl -X POST http://localhost:3000/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "Qwen/Qwen2.5-0.5B-Instruct",
    "prompt": "Hello! Tell me about Rust.",
    "max_tokens": 100
  }'
```

### 4. Try the Web UI

1. Open `http://localhost:3000` in your browser
2. Type a message in the input box
3. Watch the AI respond in real-time!

## Configuration (Optional)

To customize the service, create a `config.toml` file:

```bash
# Copy the example config
cp config.example.toml config.toml

# Edit with your favorite editor
nano config.toml  # or vim, code, etc.
```

### Common Configurations

**Change the port**:
```toml
[server]
port = 8080
```

**Use CPU instead of GPU**:
```toml
[models]
default_device = "cpu"
```

**Enable API authentication**:
```toml
[security]
enable_auth = true

[[security.api_keys]]
key = "sk-your-secret-key"
name = "my-app"
enabled = true
```

**Set rate limits**:
```toml
[limits]
default_rate_limit_per_minute = 100
max_prompt_length = 4096
```

## Testing the API

### Using cURL

**Simple completion**:
```bash
curl -X POST http://localhost:3000/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "Qwen/Qwen2.5-0.5B-Instruct",
    "prompt": "Write a haiku about programming:",
    "max_tokens": 50,
    "temperature": 0.8
  }'
```

**Chat with session**:
```bash
curl -X POST http://localhost:3000/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model-name": "Qwen/Qwen2.5-0.5B-Instruct",
    "prompt": "What is Rust?",
    "session-id": "my-session",
    "max-token": 256
  }'
```

### Using Python

```python
import requests

# Simple completion
response = requests.post(
    "http://localhost:3000/completions",
    json={
        "model": "Qwen/Qwen2.5-0.5B-Instruct",
        "prompt": "Explain async/await in Rust:",
        "max_tokens": 100
    }
)

print(response.json()["text"])
```

### Using JavaScript/Node.js

```javascript
const response = await fetch("http://localhost:3000/completions", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({
    model: "Qwen/Qwen2.5-0.5B-Instruct",
    prompt: "What are Rust's main features?",
    max_tokens: 100
  })
});

const data = await response.json();
console.log(data.text);
```

## Using Postman

Import the included Postman collection:

1. Open Postman
2. Click **Import**
3. Select `postman_collection.json` from the project root
4. Update the `baseUrl` variable to `http://localhost:3000`
5. Try the pre-configured requests!

## Monitoring

### View Metrics

```bash
curl http://localhost:3000/metrics
```

This returns Prometheus-formatted metrics including:
- Request counts
- Inference latency
- Token generation rate
- Error rates

### With Prometheus & Grafana

If you're using Docker Compose:

```bash
docker-compose up
```

Access:
- **Prometheus**: `http://localhost:9090`
- **Grafana**: `http://localhost:3002` (admin/admin)

## Common Issues

### Models Not Loading

**Problem**: First request times out or fails.

**Solution**: Models download and load on first use. This can take time:
```bash
# Check logs for download progress
cargo run --release 2>&1 | grep -i "loading\|download"
```

### CUDA Not Found

**Problem**: `CUDA requested but not available`.

**Solution**: 
- Install CUDA Toolkit 12.2+
- Or use CPU mode: `cargo run --release --bin server`

### Port Already in Use

**Problem**: `Address already in use (os error 48)`.

**Solution**: Change port in `config.toml` or stop the conflicting service:
```bash
# Find what's using port 3000
lsof -i :3000  # macOS/Linux
netstat -ano | findstr :3000  # Windows
```

### Out of Memory

**Problem**: System runs out of memory during inference.

**Solution**:
- Use smaller models
- Reduce `max_tokens` in requests
- Limit `max_concurrent_requests` in config

## Next Steps

- **Read the [API Documentation](API.md)** for full endpoint reference
- **Explore the Web UI** features (edit messages, regenerate, etc.)
- **Set up authentication** for production use
- **Configure rate limiting** to protect your service
- **Add custom models** by editing `config.toml`
- **Deploy with Docker** for production environments

## Getting Help

- **Documentation**: See [README.md](README.md) and [API.md](API.md)
- **Examples**: Check `postman_collection.json` for API examples
- **Issues**: Report bugs on GitHub
- **Logs**: Check console output for detailed error messages

## Quick Reference

| Command | Purpose |
|---------|---------|
| `cargo run --release --bin server` | Run in CPU mode |
| `cargo run --release --features cuda --bin server` | Run in GPU mode |
| `cargo test` | Run tests |
| `docker-compose up llm-cpu` | Run with Docker (CPU) |
| `curl http://localhost:3000/health` | Health check |
| `curl http://localhost:3000/models` | List models |
| `curl http://localhost:3000/metrics` | View metrics |

Happy coding! 🦀✨
