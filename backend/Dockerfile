# =============================================================================
# AeroXe Broadband Backend - Multi-stage Docker Build
# =============================================================================

# ── Build Stage ─────────────────────────────────────────────────────────────
# Rust >= 1.85 is required: some dependencies use the edition2024 crate
# feature, which is not stabilized in Cargo 1.84.
FROM rust:1.96-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Cache dependencies: copy manifests and build.rs first, create dummy src so
# only the real source is rebuilt after COPY.
COPY Cargo.toml Cargo.lock build.rs ./
RUN mkdir -p src && \
    echo 'fn main() { println!("Dummy"); }' > src/main.rs && \
    echo 'fn main() { println!("Dummy"); }' > src/bin/migrate.rs && \
    cargo build --release 2>/dev/null || true && \
    rm -rf src

# Copy actual source, build script and protos, then rebuild
COPY src/ src/
COPY migrations/ migrations/
COPY proto/ proto/
RUN touch src/main.rs && cargo build --release

# ── Runtime Stage ───────────────────────────────────────────────────────────
# trixie-slim ships glibc >= 2.38, matching the builder toolchain (rust:1.96).
FROM debian:trixie-slim

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
