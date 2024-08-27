FROM rust:1

WORKDIR /usr/src/steve_portfolio_website
COPY . .

RUN rustup target add wasm32-unknown-unknown
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall --locked trunk
RUN trunk build ./frontend/index.html
RUN cargo build --release

CMD ["./target/release/backend -b ./backend/resources -f ./frontend/dist"]
