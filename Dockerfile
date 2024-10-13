FROM rust:1-slim as builder

WORKDIR /src

# Dummy source to cache dependencies
COPY Cargo.toml Cargo.lock .
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target "$(uname -m)-unknown-linux-gnu"

# Actual source code
COPY . .
RUN RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target "$(uname -m)-unknown-linux-gnu"
RUN cp "/src/target/$(uname -m)-unknown-linux-gnu/release/blog" /usr/local/bin/blog

FROM scratch

COPY --from=builder /usr/local/bin/blog /usr/local/bin/blog

CMD ["/usr/local/bin/blog"]
