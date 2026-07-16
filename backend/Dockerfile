# Build stage
FROM rust:1.78-slim as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() { println!("Dummy"); }' > src/main.rs && cargo build --release 2>/dev/null || true
COPY src/ src/
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/aeroxe-backend /usr/local/bin/
COPY --from=builder /app/target/release/migrate /usr/local/bin/
EXPOSE 8000
CMD ["aeroxe-backend"]
