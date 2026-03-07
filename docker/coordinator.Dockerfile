FROM rust:latest

WORKDIR /app

COPY rust/coordinator .

RUN cargo build --release

CMD ["./target/release/coordinator"]
