FROM rust:1.75-slim-bookworm AS builder
WORKDIR /usr/src/rodeit
COPY packages/ packages/
COPY services/user services/user
WORKDIR /usr/src/rodeit/services/user
RUN apt-get update && apt-get install -y pkg-config libssl-dev libclang-dev g++
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/user /usr/local/bin/user
CMD ["user"]
