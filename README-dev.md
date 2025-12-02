# Rust LLM 推理服务 (Rust LLM Inference Service)

这是一个基于 Rust 构建的高性能大语言模型（LLM）推理服务，旨在提供类似 OpenAI API 的体验，支持流式输出、历史对话记忆以及 GPU 加速。

## 🚀 已实现功能 (Current Features)

1.  **高性能推理核心**:
    *   集成 `mistral.rs` 推理引擎。
    *   支持 **CUDA (NVIDIA GPU)** 加速，大幅提升推理速度。
    *   支持 CPU 回退模式（当无 GPU 时自动切换）。
    *   支持模型：`Qwen/Qwen2.5-0.5B-Instruct`, `microsoft/Phi-3.5-mini-instruct`。

2.  **Web 服务与交互**:
    *   基于 `Axum` 的高性能 Web 服务器。
    *   **WebSocket** 实时流式对话接口。
    *   **REST API** (`/chat/completions`) 支持。
    *   内置现代化 Web 前端：
        *   支持 Dark Mode 与 Markdown 渲染。
        *   **停止生成 (Stop Generation)**: 支持中途打断推理。
        *   **重新生成 (Regenerate)**: 支持重新生成满意的回复。
        *   **编辑输入 (Edit Input)**: 支持修改历史提问并重新分支对话。

3.  **会话管理 (Session Management)**:
    *   **多会话支持**: 侧边栏管理多个独立会话，支持新建、切换与删除。
    *   **持久化存储**: 对话历史保存于 `sessions.json`，重启服务不丢失。
    *   **上下文自动修剪**: 自动保留最近 20 条消息，防止上下文溢出。
    *   **历史记录回溯**: 前端自动加载之前的对话记录。

4.  **可观测性 (Observability)**:
    *   集成 **Prometheus** 指标导出 (`/metrics`)。
    *   监控指标：请求总数、推理耗时、生成 Token 数、错误率等。
    *   健康检查接口 (`/health`)。

## 🛠️ 技术实现 (Implementation Details)

*   **架构**: 采用 `Axum` 作为 Web 层，`Mistral.rs` 作为底层推理引擎。两者通过 `Arc<M1EngineAdapter>` 进行线程安全的交互。
*   **状态管理**: 使用 `AppState` 结构体管理全局状态，包括推理引擎实例、会话存储 (`Arc<Mutex<HashMap>>`) 和指标句柄。
*   **流式处理**: 利用 Rust 的 `AsyncStream` 和 `Tokio` 通道，将推理生成的 Token 实时推送到 WebSocket 或 SSE 连接。
*   **量化与精度**: 默认使用 `BF16` 精度加载模型（在 GPU 上），对于小模型（如 0.5B）禁用量化以保证最佳效果。

## 💻 如何运行 (How to Run)

### 前置要求
*   Rust (最新 Stable 版本)
*   (可选) NVIDIA 显卡驱动 & CUDA Toolkit (用于 GPU 加速)

### 启动服务

1.  **GPU 模式 (推荐)** - 速度极快:
    ```powershell
    cargo run --release --features cuda --bin server
    ```

2.  **CPU 模式** - 速度较慢:
    ```powershell
    cargo run --release --bin server
    ```

服务启动后，访问: `http://localhost:3000`

## 📅 未来计划 (Roadmap)

1.  **安全鉴权 (Security)**:
    *   实现 API Key 验证机制，保护接口不被滥用。
2.  **流量控制 (Rate Limiting)**:
    *   添加 IP 或 Token 级别的速率限制，防止资源耗尽。
3.  **配置化 (Configuration)**:
    *   引入 `config.toml`，支持自定义模型路径、端口、日志级别等。
4.  **容器化 (Dockerization)**:
    *   提供 Dockerfile 和 docker-compose 配置，简化部署流程。

---
*Last Updated: 2025-12-02*
