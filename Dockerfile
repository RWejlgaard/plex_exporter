# syntax=docker/dockerfile:1

FROM rust:1-slim-bookworm AS builder
WORKDIR /app

# rustls' default crypto provider (aws-lc-rs) compiles a native C library at
# build time and needs cmake + a C compiler, neither of which the slim base
# image ships with.
RUN apt-get update \
    && apt-get install -y --no-install-recommends build-essential cmake pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Build just the dependency graph first so it's cached across source changes.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

COPY src ./src
RUN touch src/main.rs \
    && cargo build --release

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
RUN useradd --system --no-create-home --shell /usr/sbin/nologin plex-exporter
COPY --from=builder /app/target/release/plex-exporter /usr/local/bin/plex-exporter

USER plex-exporter
EXPOSE 9000
ENTRYPOINT ["/usr/local/bin/plex-exporter"]
