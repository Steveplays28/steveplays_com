# Base
FROM docker.io/rust:1-slim-bookworm AS base

RUN rustup target add wasm32-unknown-unknown

# Chef
FROM base AS chef

RUN cargo install cargo-chef

RUN cargo install trunk

RUN --mount=target=/var/lib/apt/lists,type=cache,sharing=locked \
    --mount=target=/var/cache/apt,type=cache,sharing=locked \
    rm -f /etc/apt/apt.conf.d/docker-clean \
    && apt-get update \
    && apt-get -y --no-install-recommends install pkg-config libssl-dev
RUN cargo install sccache
ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache

WORKDIR /app

# Planner
FROM chef AS planner

COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef prepare --recipe-path recipe.json

# Builder
FROM chef AS builder

ARG server_package=backend
ARG client_package=frontend

COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --release

RUN  --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    trunk --config=$client_package/Trunk.toml build index.html

# Application
FROM debian:bookworm-slim AS runtime

ARG server_package=backend
ARG client_package=frontend

WORKDIR /app

# Copy the main binary
COPY --from=builder /app/target/release/$server_package /usr/local/bin
# Copy static assets
COPY --from=builder /build/$backend_package/Rocket.toml ./static
COPY --from=builder /build/$backend_package/static ./static
COPY --from=builder /build/$backend_package/templates ./templates
COPY --from=builder /build/$server_package/resources ./$server_package/resources
COPY --from=builder /build/$client_package/dist ./$client_package/dist

ENTRYPOINT ["/usr/local/bin/$server_package", "-b", "backend/resources", "-f", "frontend/dist"]
