FROM rust:1

WORKDIR /usr/src/steve_portfolio_website
COPY . .

RUN rustup target add wasm32-unknown-unknown && cargo install --locked trunk
RUN cd frontend && trunk build && cd ../
RUN cargo build --release

# TODO: Add configuration options for the resources and dist folders
RUN cp backend/resources target/release/resources
RUN rm -rf backend/resources
RUN cp frontend/dist target/frontend/dist
RUN rm -rf frontend/dist

CMD ["./target/release/backend"]
