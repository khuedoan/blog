FROM rust AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config openssl libssl-dev
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked cargo-leptos

COPY . .
RUN cargo leptos build --release

FROM debian:12-slim

COPY --from=builder /app/target/release/blog /app/target/release/blog
COPY --from=builder /app/target/site /app/target/site

WORKDIR /app

EXPOSE 3000

CMD ["./target/release/blog"]
