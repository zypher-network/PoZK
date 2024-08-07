FROM rust:1.76.0-buster AS builder

WORKDIR /build
ADD server server
ADD public public

WORKDIR /build/server
RUN cargo build --release


FROM ubuntu:22.04

ENV RUST_LOG=debug
ENV RUST_BACKTRACE=1
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update -y
RUN apt-get install -y apt-utils
RUN apt-get install -y libssl-dev
RUN apt-get install -y openssl

WORKDIR /app
COPY --from=builder /build/server/target/release/pozk-miner .

RUN chmod +x pozk-miner

EXPOSE 9098

ENTRYPOINT ["./pozk-miner", "file", "--path"]

