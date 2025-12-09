# Docker Deployment

This folder contains all Docker-related files for the Rust LLM Inference Service.

## Files

- **Dockerfile** - CPU-only image (works on any system)
- **Dockerfile.cuda** - GPU-accelerated image (requires NVIDIA GPU)
- **docker-compose.yml** - Complete stack with monitoring
- **prometheus.yml** - Prometheus configuration for metrics
- **.dockerignore** - Build optimization

## Quick Start

From the **project root** directory:

```bash
# Start with GPU
docker-compose -f docker/docker-compose.yml up llm-gpu

# OR start with CPU
docker-compose -f docker/docker-compose.yml up llm-cpu

# With monitoring stack
docker-compose -f docker/docker-compose.yml up -d
```

## Build Images

```bash
# CPU version
docker build -t llm-service:cpu -f docker/Dockerfile .

# GPU version
docker build -t llm-service:gpu -f docker/Dockerfile.cuda .
```

## Services

- **llm-cpu**: Port 3000 (CPU inference)
- **llm-gpu**: Port 3001 (GPU inference)
- **prometheus**: Port 9090 (Metrics)
- **grafana**: Port 3002 (Visualization, admin/admin)

## Requirements

- Docker 20.10+
- Docker Compose 1.29+
- (GPU) nvidia-docker2 and NVIDIA drivers

See main documentation for more details.
