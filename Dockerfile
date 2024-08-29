FROM rust:1

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

WORKDIR /usr/src/portfolio_website
COPY . .

RUN trunk build frontend/index.html
RUN cargo build --release

ENTRYPOINT ["target/release/backend", "-b", "backend/resources", "-f", "frontend/dist"]
