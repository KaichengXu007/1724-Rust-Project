# Multi-stage Dockerfile for Rust LLM Inference Service
# Supports both CPU and CUDA builds

# Build stage
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY public ./public

# Build release binary (CPU version by default)
# For CUDA: docker build --build-arg FEATURES=cuda
ARG FEATURES=""
RUN if [ -n "$FEATURES" ]; then \
        cargo build --release --features "$FEATURES"; \
    else \
        cargo build --release; \
    fi

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# For CUDA support, uncomment and use nvidia/cuda base image instead:
# FROM nvidia/cuda:12.2.0-runtime-ubuntu22.04
# RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/server /app/server

# Copy public web assets
COPY --from=builder /app/public /app/public

# Copy example config
COPY config.example.toml /app/config.example.toml

# Create volume mount points
VOLUME ["/app/models", "/app/data"]

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Run the binary
CMD ["./server"]
