# Rust LLM 推理服务 (Rust LLM Inference Service)

这是一个基于 Rust 构建的高性能大语言模型（LLM）推理服务，旨在提供类似 OpenAI API 的体验，支持流式输出、历史对话记忆以及 GPU 加速。

## 🚀 已实现功能 (Current Features)

### 1. **高性能推理核心**
*   集成 `mistral.rs` 推理引擎。
*   支持 **CUDA (NVIDIA GPU)** 加速，大幅提升推理速度。
*   支持 CPU 回退模式（当无 GPU 时自动切换）。
*   支持 Metal (macOS) 加速。
*   支持模型：`Qwen/Qwen2.5-0.5B-Instruct`, `microsoft/Phi-3.5-mini-instruct`。
*   懒加载模型机制：首次请求时自动加载并缓存。

### 2. **Web 服务与交互**
*   基于 `Axum` 的高性能 Web 服务器。
*   **WebSocket** 实时流式对话接口。
*   **REST API** (`/completions`, `/chat/completions`) 支持。
*   **Server-Sent Events (SSE)** 流式输出。
*   内置现代化 Web 前端：
    *   支持 Dark Mode 与 Markdown 渲染。
    *   **停止生成 (Stop Generation)**: 支持中途打断推理。
    *   **重新生成 (Regenerate)**: 支持重新生成满意的回复。
    *   **编辑输入 (Edit Input)**: 支持修改历史提问并重新分支对话。

### 3. **会话管理 (Session Management)**
*   **多会话支持**: 侧边栏管理多个独立会话，支持新建、切换与删除。
*   **持久化存储**: 对话历史保存于 `sessions.json`，重启服务不丢失。
*   **上下文自动修剪**: 自动保留最近 20 条消息，防止上下文溢出。
*   **历史记录回溯**: 前端自动加载之前的对话记录。
*   **会话回滚**: 支持回退指定数量的对话消息。

### 4. **可观测性 (Observability)**
*   集成 **Prometheus** 指标导出 (`/metrics`)。
*   监控指标：
    *   请求总数 (`*_requests_total`)
    *   推理耗时 (`*_duration_seconds`)
    *   生成 Token 数 (`*_tokens_total`)
    *   错误率 (`*_errors_total`)
*   健康检查接口 (`/health`)。
*   就绪检查接口 (`/readiness`) - 验证模型可用性。
*   结构化日志（可配置日志级别）。

### 5. **配置系统 (Configuration)**
*   **TOML 配置文件** (`config.toml`):
    *   服务器设置：主机、端口、日志级别
    *   模型配置：可用模型列表、设备选择、量化设置
    *   安全设置：API 密钥、CORS、认证开关
    *   限制设置：提示长度、响应 Token、会话数量、TTL
    *   可观测性：指标开关、追踪设置
*   **配置验证**: 启动时自动验证配置正确性。
*   **默认值支持**: 未提供配置文件时使用合理默认值。

### 6. **安全与治理 (Security & Governance)**
*   **API 密钥认证**:
    *   可选的 Bearer Token 认证
    *   每个密钥可配置独立的速率限制
    *   支持启用/禁用单个密钥
*   **速率限制**:
    *   基于 API 密钥或 IP 地址
    *   每分钟请求数可配置
    *   使用滑动窗口算法
    *   独立的限流器实现（`RateLimiter`）
*   **内容验证**:
    *   提示长度限制（默认 8192 字符）
    *   响应 Token 限制（默认 2048）
    *   自动裁剪超长请求
*   **CORS 支持**: 跨域资源共享配置。

### 7. **容器化部署 (Containerization)**
*   **多阶段 Dockerfile**:
    *   `Dockerfile`: CPU 版本
    *   `Dockerfile.cuda`: GPU/CUDA 版本
    *   优化的镜像大小
*   **Docker Compose**:
    *   CPU 和 GPU 服务配置
    *   集成 Prometheus 监控
    *   集成 Grafana 可视化
    *   卷挂载支持（模型、数据、配置）
*   **健康检查**: 容器级健康探针。

### 8. **测试覆盖 (Testing)**
*   **单元测试**:
    *   配置系统测试 (`config_tests.rs`)
    *   中间件测试 (`middleware_tests.rs`)
    *   速率限制器测试
*   **集成测试**:
    *   所有 API 端点测试 (`integration_tests.rs`)
    *   会话管理测试
    *   提示验证测试
    *   流式响应测试
*   **Mock 引擎**: 用于测试的模拟推理引擎。

### 9. **文档与工具 (Documentation & Tooling)**
*   **API 文档** (`API.md`):
    *   完整的端点参考
    *   请求/响应示例
    *   错误代码说明
    *   cURL、Python、JavaScript 示例
*   **Postman 集合** (`postman_collection.json`):
    *   预配置的 API 请求
    *   环境变量支持
    *   认证示例
*   **示例配置** (`config.example.toml`):
    *   带注释的完整配置模板
*   **README** (`README.md`):
    *   快速开始指南
    *   架构说明
    *   部署指南
    *   故障排除

## 🛠️ 技术实现 (Implementation Details)

*   **架构**: 采用 `Axum` 作为 Web 层，`Mistral.rs` 作为底层推理引擎。两者通过 `Arc<M1EngineAdapter>` 进行线程安全的交互。
*   **状态管理**: 使用 `AppState` 结构体管理全局状态，包括推理引擎实例、会话存储 (`Arc<Mutex<HashMap>>`)、指标句柄和配置。
*   **流式处理**: 利用 Rust 的 `AsyncStream` 和 `Tokio` 通道，将推理生成的 Token 实时推送到 WebSocket 或 SSE 连接。
*   **量化与精度**: 默认使用 `BF16` 精度加载模型（在 GPU 上），对于小模型（如 0.5B）禁用量化以保证最佳效果。
*   **中间件栈**:
    *   认证中间件 (`auth_middleware`)
    *   速率限制中间件 (`rate_limit_middleware`)
    *   CORS 层 (`CorsLayer`)
    *   日志追踪 (`tower-http`)

## 💻 如何运行 (How to Run)

### 前置要求
*   Rust (最新 Stable 版本)
*   (可选) NVIDIA 显卡驱动 & CUDA Toolkit (用于 GPU 加速)
*   (可选) Docker & Docker Compose (用于容器化部署)

### 启动服务

1.  **创建配置文件** (可选):
    ```powershell
    Copy-Item config.example.toml config.toml
    # 编辑 config.toml 自定义设置
    ```

2.  **GPU 模式 (推荐)** - 速度极快:
    ```powershell
    cargo run --release --features cuda --bin server
    ```

3.  **CPU 模式** - 速度较慢:
    ```powershell
    cargo run --release --bin server
    ```

4.  **Docker 运行**:
    ```powershell
    # CPU
    docker-compose up llm-cpu

    # GPU
    docker-compose up llm-gpu
    ```

服务启动后，访问: `http://localhost:3000`

### 测试

```powershell
# 运行所有测试
cargo test

# 运行集成测试
cargo test --test integration_tests

# 运行特定测试
cargo test test_completions_endpoint
```

## 📊 API 端点 (API Endpoints)

| 端点 | 方法 | 描述 |
|------|------|------|
| `/health` | GET | 健康检查 |
| `/readiness` | GET | 就绪检查 |
| `/metrics` | GET | Prometheus 指标 |
| `/models` | GET | 列出所有模型 |
| `/models/:id` | GET | 获取模型详情 |
| `/completions` | POST | 文本补全（非对话） |
| `/chat/completions` | POST | 对话补全（SSE 流式） |
| `/chat/ws` | WS | WebSocket 对话 |
| `/sessions` | GET | 列出会话 |
| `/chat/history/:id` | GET | 获取会话历史 |
| `/chat/history/:id` | DELETE | 删除会话 |
| `/chat/history/:id/rollback` | POST | 回滚会话历史 |

详细文档见 [API.md](API.md)

## 📅 路线图 (Roadmap)

### 已完成 ✅
- [x] 基础推理引擎集成
- [x] REST API 实现
- [x] WebSocket 流式对话
- [x] 会话管理与持久化
- [x] Web UI (Dark Mode, Markdown)
- [x] Prometheus 指标导出
- [x] 配置系统 (TOML)
- [x] API 密钥认证
- [x] 速率限制
- [x] 内容验证与安全防护
- [x] Docker 容器化
- [x] 测试覆盖
- [x] API 文档

### 规划中 🚧
1.  **模型热加载 (Model Hot Reload)**:
    *   动态加载/卸载模型 API
    *   模型元数据查询
2.  **高级可观测性**:
    *   Token/秒速率指标
    *   缓存命中率统计
    *   模型加载时间追踪
3.  **性能优化**:
    *   请求批处理 (Batching)
    *   KV 缓存优化
    *   并发请求限制
4.  **扩展功能**:
    *   Function calling 支持
    *   多模态模型支持
    *   模型量化配置 API

---

## 📝 更新日志

### v0.1.0 (2025-12-07)
- ✨ 初始版本发布
- ✅ 完整的 OpenAI 兼容 API
- ✅ GPU/CPU 多设备支持
- ✅ 企业级安全特性
- ✅ 生产就绪的容器化
- ✅ 全面的文档和测试

---

*Last Updated: 2025-12-07*

