# 🦀 Flux AI Terminal - Professional Docker Engine
# This Dockerfile allows Flux to be published as a GitHub Package (GHCR)

FROM rust:1.78-slim-bookworm as builder

WORKDIR /usr/src/flux
COPY . .

# Build the high-performance core
RUN cargo build --release --manifest-path core/Cargo.toml

# Production Image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/flux/core/target/release/libflux_core.so /usr/local/lib/
COPY --from=builder /usr/src/flux/assets /usr/local/share/flux/assets

ENV LD_LIBRARY_PATH=/usr/local/lib
ENV FLUX_DATA_DIR=/usr/local/share/flux/assets

LABEL org.opencontainers.image.source="https://github.com/MuhammadLutfiMuzakiiVY/flux-ai-terminal"
LABEL org.opencontainers.image.description="Ultra-High Performance Flux AI Terminal Engine"

CMD ["/bin/bash"]
