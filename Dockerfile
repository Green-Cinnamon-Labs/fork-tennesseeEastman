# ── Stage 1: build ────────────────────────────────────────────────────────────
FROM rust:latest AS builder

RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace manifests first for dependency caching
COPY tennessee-eastman-service/Cargo.toml tennessee-eastman-service/Cargo.lock ./
COPY tennessee-eastman-service/core/Cargo.toml core/Cargo.toml
COPY tennessee-eastman-service/service/Cargo.toml service/Cargo.toml

# Create dummy sources to build dependencies
RUN mkdir -p core/src && echo "pub fn dummy() {}" > core/src/lib.rs \
    && mkdir -p service/src && echo "fn main() {}" > service/src/main.rs

# Copy proto and build.rs
COPY tennessee-eastman-service/service/proto service/proto
COPY tennessee-eastman-service/service/build.rs service/build.rs

# Build dependencies only (cached unless Cargo.toml changes)
RUN cargo build --release --bin te_service 2>/dev/null || true

# Copy real source code
COPY tennessee-eastman-service/core/src core/src
COPY tennessee-eastman-service/service/src service/src

# Build the actual binary
RUN touch core/src/lib.rs service/src/main.rs && cargo build --release --bin te_service

# ── Stage 2: runtime ─────────────────────────────────────────────────────────
FROM debian:trixie-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/te_service /usr/local/bin/te_service
COPY tennessee-eastman-service/cases/te_exp3_snapshot.toml /app/cases/te_exp3_snapshot.toml

WORKDIR /app

EXPOSE 50051

ENTRYPOINT ["te_service", "--headless"]
