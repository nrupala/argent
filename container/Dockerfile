# ARGENT Zero-Knowledge Container
# Multi-stage build for minimal attack surface

# Stage 1: Build
FROM rust:1.86-bookworm AS builder

WORKDIR /build

RUN apt-get update && apt-get install -y \
    clang \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# Stage 2: Runtime - Minimal base
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    tzdata \
    && useradd -m -s /bin/bash argent \
    && mkdir -p /home/argent/workspace \
    && chown -R argent:argent /home/argent \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

COPY --from=builder /build/target/release/argent /usr/local/bin/argent
COPY --chown=argent:argent configs/ /home/argent/.argent/

USER argent
WORKDIR /home/argent/workspace

ENV ARGENT_STORAGE=/home/argent/.argent
ENV ARGENT_KEY_VAR=ARGENT_KEY
ENV RUST_LOG=info

ENTRYPOINT ["argent"]
CMD ["--help"]

# Security hardening
RUN chmod 500 /home/argent/.argent && \
    chmod 500 /home/argent/workspace

# Labels
LABEL maintainer="Argent Team"
LABEL version="0.1.0"
LABEL description="Zero-knowledge autonomous coding agent"
