FROM rust as builder

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config openssl libssl-dev

RUN cargo install --git https://github.com/leptos-rs/cargo-leptos cargo-leptos

# TODO optimize build speed and image size
COPY . .

RUN cargo leptos build --release

FROM debian:12-slim

COPY --from=builder /app/target/server/release/blog /app/target/server/release/blog
COPY --from=builder /app/target/site /app/target/site

WORKDIR /app

ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"

EXPOSE 3000

CMD ["./target/server/release/blog"]
