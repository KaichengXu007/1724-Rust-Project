## Project Proposal

### Motivation

**Large language models (LLMs)** are the engines behind modern natural language processing, powering applications such as conversational agents, code assistants, and intelligent search. However, as these models continue to scale in size and capability, they have become **increasingly resource-intensive, creating bottlenecks for real-time applications**. Efficient **inference infrastructure** is therefore essential, yet current deployments often face challenges such as high memory consumption, long response times, and the need to coordinate multiple hardware resources — **making large-scale LLM deployment in production extremely complex**.

Most existing inference systems are built using **Python-based frameworks** such as PyTorch or TensorFlow. While powerful, these frameworks often **struggle under heavy load, suffering from runtime overhead, limited concurrency**, and unpredictable latency. For applications that demand low latency and real-time streaming — such as chatbots or code assistants — these limitations are critical.

The **Rust programming language** offers a compelling alternative for AI infrastructure. It combines the performance of C++ with memory safety and superior concurrency, making it ideal for building stable, high-performance inference systems. Rust’s **zero-cost abstractions** and **fine-grained concurrency model** make it a strong candidate for next-generation LLM serving frameworks.

Our motivation is to **bridge this gap** by building a **Rust-based LLM inference service** that emphasizes efficiency, scalability, and stability. Frameworks like **Candle** and **Burn** provide early-stage support for model execution, while backend technologies such as **Axum** and **Rocket** will support high-performance request handling and streaming responses.

Beyond technical exploration, this project is **ecosystem-driven**. It offers hands-on experience in systems programming, web backend design, and machine learning deployment. We are particularly excited by the challenge of combining **low-level performance with modern AI applications** — a satisfying and rewarding intersection of systems design and artificial intelligence.

### Objective

Build a **Rust-native Large Language Model (LLM) inference service** that can **load, manage, and serve multiple local LLMs** with **token streaming**, a clean **WebSocket API**, and a **minimal chat UI**. The service prioritizes portability (single binary + model files), safety, and observability, filling a gap in the Rust ecosystem where most open-source inference stacks are Python-first.

### Key Features

1. **Model loading & registry**

   * Load GGUF-supported formats via Candle

   * Hot-register, list, unload models; configure quantization & device.

2. **Inference API**

   * REST endpoints: `/models`, `/completions`, `/chat/completions`.

   * Request options: temperature, top-p, max tokens, stop sequences, system prompt.

3. **Streaming tokens**

   * Server-Sent Events and WebSocket streaming for low-latency outputs.

4. **Session & context handling**

   - Lightweight conversation state with configurable memory limits.

5. **Basic web chat**

   - Minimal front-end to select a model and interact with the API; shows live streaming output.

6. **Observability**

   - Structured logs; Prometheus metrics (latency, tokens/s, cache hits); health/readiness probes.

7. **Security & governance**

   - API key auth; per-key rate limiting; prompt/response length guards; safe defaults.

8. **Packaging & DX**

   - One-command run (Cargo + config file); Dockerfile; example configs and Postman collection.

9. **Testing**

   - Unit tests for tokenization & sampling; integration tests for API and streaming paths.

### Tentative Plan

#### 1. Roles & Responsibilities

The team will be structured around three specialized roles, ensuring clear ownership over the core components: the high-performance inference core, the robust web API, and the integration/deployment layer.

-   **M1: Inference Engine Specialist (AI/Infra).** M1 owns the **core model logic** and performance. This includes selecting and integrating **Candle/Mistral.rs**, implementing GGUF model loading, managing **model concurrency and scheduling**, optimizing the iterative token generation loop, and developing unit tests for tokenization and sampling.
    
-   **M2: Backend Service Architect (Backend/API).** M2 owns the **API architecture and data flow**. This involves building the **Axum server**, implementing the REST endpoints (`/models`, `/completions`), defining and implementing the **streaming endpoints** (SSE and WebSocket), managing **session/context handling**, and implementing security features like **API key auth and rate limiting**.
    
-   **M3: Full-stack Integration Engineer (Full-stack/DevOps).** M3 owns the user experience and production readiness. This involves building the **minimal Web Chat UI**, integrating the streaming API with the frontend, setting up the **Observability** stack (structured logs, Prometheus metrics, health probes), and preparing the final **Packaging & DX** (Dockerfile, one-command run, documentation).

#### 2. Tentative Work Plan (8 Weeks)

#### Phase 1: Core Inference Backbone (Weeks 1-2)

The primary goal of this phase is to establish a working, non-streaming core service. **M1** will select the final inference framework and implement the basic **GGUF model loading** and a synchronous (blocking) text generation function. **M2** will build the basic **Axum server** structure, define the shared application state for model access, and expose the `/models` endpoint for listing models. **M3** will initialize the project with boilerplate code and configure **structured logging**. The key deliverable is a functional `/completions` REST endpoint that returns a full, non-streaming response for a single model.

#### Phase 2: Concurrency and Model Management (Weeks 3-4)

This phase focuses on performance and resource control. **M1** will refactor the model access to be **concurrency-safe**, likely using Rust's `Arc` and asynchronous **MPSC channels** or other locking mechanisms to manage the queue of inference requests efficiently. **M2** will implement the logic for **hot-registering, unloading, and managing multiple models** and will integrate the **session and context handling** logic to maintain conversation history. The deliverable is a robust service capable of safely and efficiently handling multiple concurrent client requests against shared or multiple LLM resources.

#### Phase 3: Streaming API and Chat UI (Weeks 5-7)

This is the feature-intensive phase. **M1** must adapt the core logic to **iteratively yield tokens**, enabling streaming. **M2** will implement the low-latency **streaming tokens API**, prioritizing the **Server-Sent Events (SSE)** endpoint (`/chat/completions`) as it’s often simpler for a first implementation. **M3** will concurrently build the **Minimal Web Chat UI** and implement the client-side logic (using the browser's `EventSource` API) to consume and display the real-time token stream. The deliverable is a fully working, end-to-end chat experience with live streaming output.

#### Phase 4: Production Readiness and Finalization (Week 8)

The final week is dedicated to hardening the service. **M3** will integrate **Prometheus metrics** (latency, tokens/s, cache hits) and implement basic **health/readiness probes** for deployment readiness. **M2** will implement the required **Security & governance** features: **API key authentication** and **per-key rate limiting**. Both **M1 and M2** will focus on writing comprehensive **unit and integration tests** across the inference, API, and streaming paths. Finally, **M3** will finalize the **Dockerfile**, configuration files, and documentation to ensure a clean, one-command run developer experience.
