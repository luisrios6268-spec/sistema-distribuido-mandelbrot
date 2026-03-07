FROM rust:latest

WORKDIR /app

COPY rust/worker .

RUN cargo build --release

CMD ["./target/release/worker"]
