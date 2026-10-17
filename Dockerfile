FROM rust:1.99-bookworm AS builder

# aws-lc-sys (TLS via reqwest/rustls) needs cmake and clang to build
RUN apt-get update && apt-get install -y --no-install-recommends cmake clang \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/rss-updates /usr/local/bin/rss-updates
# migrations are read at runtime from ./src/migrations
COPY src/migrations ./src/migrations

EXPOSE 3000
CMD ["rss-updates"]
