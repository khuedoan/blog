FROM rust:1-slim as builder

WORKDIR /src

# Dummy source to cache dependencies
COPY Cargo.toml Cargo.lock .
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu

# Actual source code
COPY . .
RUN RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu

FROM scratch

COPY --from=builder /src/target/x86_64-unknown-linux-gnu/release/blog /usr/local/bin/blog

CMD ["/usr/local/bin/blog"]
