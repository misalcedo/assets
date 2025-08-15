FROM rust:latest AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/* /usr/local/bin/
WORKDIR /var/assets
RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*
CMD ["assets", "start", "-d", "assets.db", "-a", "0.0.0.0:2738"]