# API Reference

Complete API documentation for the Rust LLM Inference Service.

**Base URL**: `http://localhost:3000`

---

## Table of Contents

- [Authentication](#authentication)
- [Health & Monitoring](#health--monitoring)
- [Models](#models)
- [Completions](#completions)
- [Chat Completions](#chat-completions)
- [WebSocket Chat](#websocket-chat)
- [Session Management](#session-management)
- [Error Handling](#error-handling)
- [Rate Limiting](#rate-limiting)
- [Examples](#examples)

---

## Authentication

If authentication is enabled in `config.toml`, include an API key:

```http
Authorization: Bearer YOUR_API_KEY
```

---

## Health & Monitoring

### GET /health
Health check endpoint.

**Response**:
```json
{
  "status": "ok",
  "uptime": "running",
  "timestamp": "2025-12-07T10:30:00Z"
}
```

### GET /readiness
Readiness check (verifies models are loaded).

**Response**:
```json
{
  "status": "ready",
  "models_available": 2,
  "timestamp": "2025-12-07T10:30:00Z"
}
```

### GET /metrics
Prometheus-compatible metrics endpoint.

**Metrics Include**:
- `health_check_requests_total` - Health check count
- `completions_requests_total` - Completion requests
- `chat_completions_requests_total` - Chat requests
- `completions_duration_seconds` - Inference latency
- `completions_tokens_total` - Tokens generated
- `completions_errors_total` - Error count

---

## Models

### GET /models
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

### GET /models/:model_id
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

## Completions

### POST /completions
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
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `model` | string | Yes | - | Model name |
| `prompt` | string | Yes | - | Input prompt |
| `max_tokens` | integer | No | 128 | Max tokens to generate |
| `temperature` | float | No | 0.7 | Sampling temperature (0-2) |
| `top_p` | float | No | 0.95 | Nucleus sampling probability |
| `stop` | array | No | [] | Stop sequences |
| `stream` | boolean | No | false | Enable streaming |

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

## Chat Completions

### POST /chat/completions
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
  "top-k": 40,
  "repeat-penalty": 1.1,
  "system-prompt": "You are a helpful assistant",
  "stop": [],
  "device": "cuda"
}
```

**Parameters**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `model-name` | string | Yes | - | Model name |
| `prompt` | string | Yes | - | User message |
| `session-id` | string | No | auto | Session ID for context |
| `max-token` | integer | No | 512 | Max tokens |
| `temperature` | float | No | 0.7 | Temperature (0-2) |
| `top-p` | float | No | 0.95 | Top-p sampling |
| `top-k` | integer | No | 40 | Top-k sampling |
| `repeat-penalty` | float | No | 1.1 | Repetition penalty (1-2) |
| `system-prompt` | string | No | - | System instruction |
| `stop` | array | No | [] | Stop sequences |
| `device` | string | No | "cpu" | Device: cuda/cpu/metal |

**Response**: Server-Sent Events (SSE) stream
```
data: Rust
data:  is
data:  a
data:  systems
data:  programming
data:  language
```

---

## WebSocket Chat

### WS /chat/ws
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
  "top-p": 0.95,
  "top-k": 40,
  "repeat-penalty": 1.1,
  "system-prompt": "You are helpful",
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

## Session Management

### GET /sessions
List all active session IDs.

**Response**:
```json
["session-uuid-1", "session-uuid-2"]
```

### GET /chat/history/:session_id
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

### DELETE /chat/history/:session_id
Delete a session and its history.

**Response**: 204 No Content

### POST /chat/history/:session_id/rollback
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

## Error Handling

### Error Response Format
```json
{
  "error": "Error description here"
}
```

### HTTP Status Codes

| Code | Meaning | Common Causes |
|------|---------|---------------|
| 200 | OK | Request successful |
| 204 | No Content | Deletion successful |
| 400 | Bad Request | Invalid parameters, prompt too long |
| 401 | Unauthorized | Invalid API key |
| 429 | Too Many Requests | Rate limit exceeded |
| 500 | Internal Server Error | Inference failed, model load error |

---

## Rate Limiting

Rate limits are applied per API key or IP address:
- Default: 60 requests/minute (configurable in `config.toml`)
- Per-key limits can be customized

**Rate Limit Headers**:
```http
X-RateLimit-Limit: 60
X-RateLimit-Remaining: 45
X-RateLimit-Reset: 1609459200
```

**Rate Limit Exceeded Response**:
```http
HTTP 429 Too Many Requests
Rate limit exceeded
```

---

## Examples

### cURL Examples

**Health Check**:
```bash
curl http://localhost:3000/health
```

**List Models**:
```bash
curl http://localhost:3000/models
```

**Simple Completion**:
```bash
curl -X POST http://localhost:3000/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "Qwen/Qwen2.5-0.5B-Instruct",
    "prompt": "Explain Rust in one sentence:",
    "max_tokens": 50
  }'
```

**Chat Completion with Full Parameters**:
```bash
curl -X POST http://localhost:3000/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model-name": "Qwen/Qwen2.5-0.5B-Instruct",
    "prompt": "What is async/await in Rust?",
    "max-token": 256,
    "temperature": 0.8,
    "top-p": 0.95,
    "top-k": 40,
    "repeat-penalty": 1.1,
    "system-prompt": "You are a Rust expert",
    "device": "cuda"
  }'
```

**With API Key**:
```bash
curl -X POST http://localhost:3000/completions \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "Qwen/Qwen2.5-0.5B-Instruct",
    "prompt": "Hello",
    "max_tokens": 50
  }'
```

**Get Session History**:
```bash
curl http://localhost:3000/chat/history/my-session-id
```

**Delete Session**:
```bash
curl -X DELETE http://localhost:3000/chat/history/my-session-id
```

### Python Example

```python
import requests

# Simple completion
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

# Chat with full parameters
response = requests.post(
    "http://localhost:3000/chat/completions",
    json={
        "model-name": "Qwen/Qwen2.5-0.5B-Instruct",
        "prompt": "Explain ownership in Rust",
        "session-id": "my-session",
        "max-token": 256,
        "temperature": 0.7,
        "top-p": 0.95,
        "top-k": 40,
        "repeat-penalty": 1.1,
        "system-prompt": "You are a Rust teacher",
        "device": "cuda"
    },
    stream=True
)

for line in response.iter_lines():
    if line:
        print(line.decode('utf-8').replace('data: ', ''), end='')
```

### JavaScript/TypeScript Example

```typescript
// Simple completion
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

// Streaming chat
const chatResponse = await fetch("http://localhost:3000/chat/completions", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({
    "model-name": "Qwen/Qwen2.5-0.5B-Instruct",
    "prompt": "What is Rust?",
    "max-token": 256,
    "temperature": 0.7,
    "device": "cuda"
  })
});

const reader = chatResponse.body.getReader();
const decoder = new TextDecoder();

while (true) {
  const { done, value } = await reader.read();
  if (done) break;
  
  const chunk = decoder.decode(value);
  const lines = chunk.split('\n');
  
  for (const line of lines) {
    if (line.startsWith('data: ')) {
      process.stdout.write(line.substring(6));
    }
  }
}
```

### WebSocket Client Example

```javascript
const ws = new WebSocket("ws://localhost:3000/chat/ws");

ws.onopen = () => {
  ws.send(JSON.stringify({
    "model-name": "Qwen/Qwen2.5-0.5B-Instruct",
    "prompt": "Tell me a joke about Rust",
    "session-id": crypto.randomUUID(),
    "max-token": 100,
    "temperature": 0.8,
    "top-p": 0.95,
    "top-k": 40,
    "repeat-penalty": 1.1,
    "system-prompt": "You are a funny comedian",
    "device": "cuda"
  }));
};

ws.onmessage = (event) => {
  if (event.data.startsWith("__ERROR__")) {
    console.error("Error:", event.data);
  } else {
    process.stdout.write(event.data);
  }
};

ws.onclose = () => console.log("\nConnection closed");
ws.onerror = (error) => console.error("WebSocket error:", error);
```

---

## Web UI

Access the built-in web interface at `http://localhost:3000`

**Features**:
- Dark mode with modern design
- Real-time streaming responses
- Multi-session management
- Advanced settings panel:
  - Temperature (0-2)
  - Top-P (0-1)
  - Top-K (1-100)
  - Max Tokens (128-2048)
  - Repeat Penalty (1-2)
  - System Prompt
  - Stop Sequences
- Syntax highlighting for code blocks
- Copy button for code snippets
- Live token counter with tokens/s
- Connection status indicator
- Message regeneration
- Input editing with conversation branching
- Export conversation history

---

*Last Updated: 2025-12-07*
