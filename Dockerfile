FROM rust:latest

WORKDIR /usr/src/expenses

COPY . .

RUN cargo build --release

RUN cargo install --path .

CMD ["expenses"]
