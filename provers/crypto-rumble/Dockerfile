# Builder
FROM cryptorumble/materials-crypto-rumble-30:v1 AS base
FROM rust:bullseye AS builder

RUN update-ca-certificates
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

WORKDIR /prover
COPY --from=base /data/* ./materials/

RUN git clone https://github.com/CryptoRumble/crypto-rumble-pozk.git && cd crypto-rumble-pozk/prover && cargo update && cargo build --release && mv /prover/crypto-rumble-pozk/prover/target/release/prover /prover/

# Final image
FROM debian:bullseye-slim

WORKDIR /prover

# Copy our build
COPY --from=builder /prover/prover .

ENTRYPOINT ["./prover"]
