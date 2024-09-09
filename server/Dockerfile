# Builder
FROM rust:bullseye AS builder
RUN update-ca-certificates
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

WORKDIR /build
ADD server server
ADD public public

WORKDIR /build/server
RUN cargo build --release

# Final image
FROM debian:bullseye-slim

ENV RUST_LOG=info
ENV RUST_BACKTRACE=1
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update &&  \
    apt-get install -y --no-install-recommends apt-utils && \
    apt-get install -y --no-install-recommends libssl-dev && \
    apt-get install -y --no-install-recommends openssl

WORKDIR /app
COPY --from=builder /build/server/target/release/pozk-miner .

ENTRYPOINT ["./pozk-miner"]
