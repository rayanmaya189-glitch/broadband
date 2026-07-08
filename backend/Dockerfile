# ═══════════════════════════════════════════════════════════════
# AeroXe Backend — Multi-stage Docker Build
# ═══════════════════════════════════════════════════════════════

# Build stage
FROM rust:1.78-slim as builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release 2>/dev/null || true
RUN rm -rf src

# Build application
COPY src/ src/
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd -r aeroxe && useradd -r -g aeroxe aeroxe

COPY --from=builder /app/target/release/aeroxe-backend /usr/local/bin/

RUN chown aeroxe:aeroxe /usr/local/bin/aeroxe-backend

USER aeroxe

EXPOSE 8000

HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:8000/health || exit 1

CMD ["aeroxe-backend"]
