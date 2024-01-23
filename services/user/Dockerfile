FROM debian:bookworm-slim
LABEL authors="fabiankrusch"

WORKDIR /home

COPY target/debug/authentication-service .

ENTRYPOINT ["/home/authentication-service"]