FROM rust:1

WORKDIR /usr/src/steve_portfolio_website
COPY . .

RUN cargo install --path .
RUN cd frontend && trunk build && cd ../

CMD ["cd backend && cargo run"]