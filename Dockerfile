FROM docker.io/rust:1-slim-bookworm AS build

# Set the Cargo package name
ARG pkg=backend

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

WORKDIR /build
COPY . .

RUN --mount=type=cache,target=/build/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
	trunk build frontend/index.html; \
    cargo build --release; \
	mkdir ./release; \
	cp target/release/$pkg ./release;

################################################################################

FROM docker.io/debian:bookworm-slim

# Set the Cargo package name
ARG pkg=backend

WORKDIR /app

## Copy the main binary
COPY --from=build /build/release/$pkg ./

## Copy runtime assets which may or may not exist
COPY --from=build /build/Rocket.tom[l] ./static
COPY --from=build /build/stati[c] ./static
COPY --from=build /build/template[s] ./templates

## Copy static assets
COPY --from=build /build/backend/resources ./backend/
COPY --from=build /build/frontend/dist ./frontend/

ENTRYPOINT ["release/$pkg", "-b", "backend/resources", "-f", "frontend/dist"]
