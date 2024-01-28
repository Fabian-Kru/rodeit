FROM rust:1.75-slim-bookworm AS builder

ARG DEBIAN_FRONTEND=noninteractive

RUN rustup target add x86_64-unknown-linux-gnu
RUN rustup toolchain install stable-x86_64-unknown-linux-gnu

WORKDIR /usr/src/rodeit
COPY packages/ packages/
COPY services/bucket_list services/bucket_list

WORKDIR /usr/src/rodeit/services/bucket_list

RUN apt-get update && \
	apt-get install -y pkg-config libssl-dev libclang-dev g++ && \
	cargo install --path . && \
	apt-get purge -y pkg-config libssl-dev libclang-dev g++ && \
	apt-get clean && \
	rm -rf /var/lib/apt/lists/*

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/bucket_list /usr/local/bin/bucket_list
CMD ["bucket_list"]

