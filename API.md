# Rust LLM Inference Service - API Documentation

## Overview

The Rust LLM Inference Service provides OpenAI-compatible REST and WebSocket APIs for local LLM inference with streaming support.

**Base URL**: `http://localhost:3000`

---

## Authentication

If authentication is enabled in `config.toml`, include an API key in the `Authorization` header:

```
Authorization: Bearer YOUR_API_KEY
```

---

## Endpoints

### Health & Status

#### GET /health
Health check endpoint.

**Response**:
```json
{
  "status": "ok",
  "uptime": "running",
  "timestamp": "2025-12-07T10:30:00Z"
}
```

#### GET /readiness
Readiness check (verifies models are loaded).

**Response**:
```json
{
  "status": "ready",
  "models_available": 2,
  "timestamp": "2025-12-07T10:30:00Z"
}
```

#### GET /metrics
Prometheus-compatible metrics endpoint.

**Response**: Prometheus text format

---

### Models

#### GET /models
List all available models.

**Response**:
```json
{
  "models": [
    "Qwen/Qwen2.5-0.5B-Instruct",
    "microsoft/Phi-3.5-mini-instruct"
  ]
}
```

#### GET /models/:model_id
Get detailed information about a specific model.

**Parameters**:
- `model_id` (path): Model ID or name

**Response**:
```json
{
  "id": "qwen",
  "name": "Qwen/Qwen2.5-0.5B-Instruct",
  "context_length": 4096,
  "quantization": null
}
```

---

### Completions

#### POST /completions
Generate text completions (non-chat format).

**Request Body**:
```json
{
  "model": "Qwen/Qwen2.5-0.5B-Instruct",
  "prompt": "Once upon a time",
  "max_tokens": 100,
  "temperature": 0.7,
  "top_p": 0.95,
  "stop": ["\n\n"],
  "stream": false
}
```

**Parameters**:
- `model` (string, required): Model name
- `prompt` (string, required): Input prompt
- `max_tokens` (integer, optional): Maximum tokens to generate (default: 128)
- `temperature` (float, optional): Sampling temperature 0-2 (default: 0.7)
- `top_p` (float, optional): Nucleus sampling probability (default: 0.95)
- `stop` (array, optional): Stop sequences
- `stream` (boolean, optional): Enable streaming (default: false)

**Response (non-streaming)**:
```json
{
  "text": "Once upon a time, in a faraway land...",
  "model": "Qwen/Qwen2.5-0.5B-Instruct",
  "tokens": 15
}
```

**Response (streaming)**: Server-Sent Events (SSE)
```
data: Once
data:  upon
data:  a
data:  time
```

---

### Chat Completions

#### POST /chat/completions
Chat-style completions with conversation history.

**Request Body**:
```json
{
  "model-name": "Qwen/Qwen2.5-0.5B-Instruct",
  "prompt": "What is Rust?",
  "session-id": "optional-session-uuid",
  "max-token": 256,
  "temperature": 0.7,
  "top-p": 0.95,
  "top-k": 10,
  "repeat-penalty": 1.0,
  "stop": [],
  "device": "cuda"
}
```

**Parameters**:
- `model-name` (string, required): Model name
- `prompt` (string, required): User message
- `session-id` (string, optional): Session ID for context persistence
- `max-token` (integer, optional): Max tokens (default: 128)
- `temperature` (float, optional): Temperature (default: 0.7)
- `top-p` (float, optional): Top-p sampling (default: 0.95)
- `top-k` (integer, optional): Top-k sampling (default: 10)
- `repeat-penalty` (float, optional): Repetition penalty (default: 1.0)
- `stop` (array, optional): Stop sequences
- `device` (string, optional): Device: "cuda", "cpu", "metal" (default: "cpu")

**Response**: Server-Sent Events (SSE) stream

```
data: Rust
data:  is
data:  a
data:  systems
...
```

---

### WebSocket Chat

#### WS /chat/ws
WebSocket endpoint for real-time streaming chat.

**Connection**: `ws://localhost:3000/chat/ws`

**Send Message** (JSON):
```json
{
  "model-name": "Qwen/Qwen2.5-0.5B-Instruct",
  "prompt": "Hello!",
  "session-id": "my-session-id",
  "max-token": 256,
  "temperature": 0.7,
  "device": "cuda"
}
```

**Receive Messages** (Text): Stream of tokens
```
Hello
!
 How
 can
 I
 help
 you
?
```

**Error Format**:
```
__ERROR__:Error message here
```

---

### Session Management

#### GET /sessions
List all active session IDs.

**Response**:
```json
["session-uuid-1", "session-uuid-2"]
```

#### GET /chat/history/:session_id
Retrieve conversation history for a session.

**Response**:
```json
[
  {
    "role": "system",
    "content": "You are a helpful AI assistant."
  },
  {
    "role": "user",
    "content": "Hello"
  },
  {
    "role": "assistant",
    "content": "Hi! How can I help you?"
  }
]
```

#### DELETE /chat/history/:session_id
Delete a session and its history.

**Response**: 204 No Content

#### POST /chat/history/:session_id/rollback
Rollback conversation history by N messages.

**Request Body**:
```json
{
  "amount": 2
}
```

**Response**:
```json
{
  "status": "ok"
}
```

---

## Rate Limiting

Rate limits are applied per API key or IP address:
- Default: 60 requests/minute (configurable)
- Per-key limits can be set in `config.toml`

**Rate Limit Exceeded Response**:
```
HTTP 429 Too Many Requests
Rate limit exceeded
```

---

## Error Responses

Standard error format:
```json
{
  "error": "Error description here"
}
```

**Common Error Codes**:
- `400` - Bad Request (invalid parameters, prompt too long)
- `401` - Unauthorized (invalid API key)
- `429` - Too Many Requests (rate limit exceeded)
- `500` - Internal Server Error (inference failed)

---

## Configuration

See `config.example.toml` for full configuration options:
- Server port and host
- Model paths and devices
- API keys and rate limits
- Prompt/response length limits
- Session management settings

---

## Metrics

Prometheus metrics available at `/metrics`:

**Key Metrics**:
- `health_check_requests_total` - Health check count
- `models_list_requests_total` - Model list requests
- `completions_requests_total` - Completion requests
- `chat_completions_requests_total` - Chat completion requests
- `completions_duration_seconds` - Inference latency histogram
- `completions_tokens_total` - Total tokens generated
- `chat_inference_duration_seconds` - Chat inference latency
- `chat_generated_tokens_total` - Chat tokens generated
- `completions_errors_total` - Error count
- `chat_completions_errors_total` - Chat error count

---

## Examples

### cURL Examples

**Get models**:
```bash
curl http://localhost:3000/models
```

**Generate completion**:
```bash
curl -X POST http://localhost:3000/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "Qwen/Qwen2.5-0.5B-Instruct",
    "prompt": "Explain Rust in one sentence:",
    "max_tokens": 50
  }'
```

**Chat completion with streaming**:
```bash
curl -X POST http://localhost:3000/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model-name": "Qwen/Qwen2.5-0.5B-Instruct",
    "prompt": "What is async/await in Rust?",
    "max-token": 256,
    "device": "cuda"
  }'
```

**With API key**:
```bash
curl -X POST http://localhost:3000/completions \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"model": "Qwen/Qwen2.5-0.5B-Instruct", "prompt": "Hello"}'
```

### Python Example

```python
import requests

response = requests.post(
    "http://localhost:3000/completions",
    json={
        "model": "Qwen/Qwen2.5-0.5B-Instruct",
        "prompt": "Write a haiku about Rust:",
        "max_tokens": 50,
        "temperature": 0.8
    }
)

print(response.json()["text"])
```

### JavaScript/TypeScript Example

```typescript
const response = await fetch("http://localhost:3000/completions", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({
    model: "Qwen/Qwen2.5-0.5B-Instruct",
    prompt: "Hello, world!",
    max_tokens: 50
  })
});

const data = await response.json();
console.log(data.text);
```

---

## WebSocket Client Example

```javascript
const ws = new WebSocket("ws://localhost:3000/chat/ws");

ws.onopen = () => {
  ws.send(JSON.stringify({
    "model-name": "Qwen/Qwen2.5-0.5B-Instruct",
    "prompt": "Tell me a joke",
    "session-id": crypto.randomUUID(),
    "max-token": 100
  }));
};

ws.onmessage = (event) => {
  if (event.data.startsWith("__ERROR__")) {
    console.error("Error:", event.data);
  } else {
    process.stdout.write(event.data);
  }
};
```
