# =============================================================================
# AeroXe Broadband Backend - Multi-stage Docker Build
# =============================================================================

# ── Build Stage ─────────────────────────────────────────────────────────────
FROM rust:1.78-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Cache dependencies: copy manifests first, create dummy src, build deps
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo 'fn main() { println!("Dummy"); }' > src/main.rs && \
    echo 'fn main() { println!("Dummy"); }' > src/bin/migrate.rs && \
    cargo build --release 2>/dev/null || true && \
    rm -rf src

# Copy actual source and rebuild
COPY src/ src/
RUN touch src/main.rs && cargo build --release

# ── Runtime Stage ───────────────────────────────────────────────────────────
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y \
        ca-certificates \
        libssl3 \
        curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd -r aeroxe && useradd -r -g aeroxe -s /bin/false aeroxe

WORKDIR /app

# Copy binaries from builder
COPY --from=builder /app/target/release/aeroxe-backend /usr/local/bin/
COPY --from=builder /app/target/release/migrate /usr/local/bin/

# Set ownership
RUN chown -R aeroxe:aeroxe /app

USER aeroxe

EXPOSE 8000

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:8000/health || exit 1

ENTRYPOINT ["aeroxe-backend"]
