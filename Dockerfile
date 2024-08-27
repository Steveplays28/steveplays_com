FROM rust:1

RUN rustup target add wasm32-unknown-unknown
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall --no-confirm --locked trunk

WORKDIR /usr/src/portfolio_website
COPY . .

RUN trunk build ./frontend/index.html
RUN cargo build --release

CMD ["./target/release/backend", "-b ./backend/resources", "-f ./frontend/dist"]
