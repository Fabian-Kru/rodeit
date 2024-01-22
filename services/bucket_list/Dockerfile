FROM rust:1.75-slim-bookworm AS builder
WORKDIR /usr/src/rodeit
COPY packages/captain_coaster packages/captain_coaster
COPY services/bucket_list services/bucket_list
WORKDIR /usr/src/rodeit/services/bucket_list
RUN apt-get update && apt-get install -y pkg-config libssl-dev libclang-dev g++
RUN rustup component add rustfmt
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/bucket_list /usr/local/bin/bucket_list
CMD ["bucket_list"]

