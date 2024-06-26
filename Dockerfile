FROM rust:1.67 as builder

WORKDIR /usr/src/echo-server
COPY . .

RUN cargo build --release
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/echo-server /usr/local/bin/echo-server
CMD ["echo-server"]
