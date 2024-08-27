FROM rust:1

WORKDIR /usr/src/steve_portfolio_website
COPY . .

RUN rustup target add wasm32-unknown-unknown && cargo install --locked trunk
RUN cd frontend && trunk build && cd ../
RUN cargo build --release

CMD ["./target/release/backend -b backend/resources -f frontend/dist"]
