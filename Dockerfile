FROM rust:1

WORKDIR /usr/src/steve_portfolio_website
COPY . .

RUN rustup target add wasm32-unknown-unknown && cargo install --locked trunk
RUN cd frontend && trunk build && cd ../
RUN cargo build --release

# TODO: Add a configuration option for the resources folder
RUN cp -r backend/resources target/release/resources && rm -rf backend/resources
# TODO: Add a configuration option for the dist folder
RUN cp -r frontend/dist target/frontend/dist && rm -rf frontend/dist

CMD ["./target/release/backend"]
