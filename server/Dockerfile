FROM rust:1.76.0-buster AS builder

WORKDIR /build
ADD server server
ADD public public

WORKDIR /build/server
RUN cargo build --release


FROM debian:buster

ENV RUST_LOG=debug
ENV RUST_BACKTRACE=1
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update &&  \
    apt-get install -y --no-install-recommends apt-utils && \
    apt-get install -y --no-install-recommends libssl-dev && \
    apt-get install -y --no-install-recommends openssl


WORKDIR /app
COPY --from=builder /build/server/target/release/pozk-miner .

RUN chmod +x pozk-miner

ENTRYPOINT ["./pozk-miner", "file", "--path"]

