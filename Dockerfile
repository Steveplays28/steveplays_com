FROM docker.io/rust:1-slim-bookworm AS build

ARG backend_package=backend
ARG frontend_package=frontend

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

WORKDIR /build
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
	trunk build $frontend_package/index.html; \
    cargo build --release;

################################################################################

FROM docker.io/debian:bookworm-slim

ARG backend_package=backend
ARG frontend_package=frontend

WORKDIR /app

## Copy the main binary
COPY --from=build /build/target/release/$backend_package ./release/$backend_package

## Copy runtime assets which may or may not exist
COPY --from=build /build/Rocket.tom[l] ./static
COPY --from=build /build/stati[c] ./static
COPY --from=build /build/template[s] ./templates

## Copy static assets
COPY --from=build /build/$backend_package/resources ./$backend_package/resources
COPY --from=build /build/$frontend_package/dist ./$frontend_package/dist

ENTRYPOINT ["release/backend", "-b", "backend/resources", "-f", "frontend/dist"]
