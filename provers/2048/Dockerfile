# Builder
FROM zyphernetwork/materials-2048:v1 AS base
FROM rust:bullseye AS builder

RUN update-ca-certificates
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

WORKDIR /prover
COPY --from=base /data/* ./materials/

RUN git clone https://github.com/zypher-network/pozk-2048.git && cd pozk-2048/prover && cargo update && cargo build --release && mv /prover/pozk-2048/prover/target/release/prover /prover/

# Final image
FROM debian:bullseye-slim

WORKDIR /prover

# Copy our build
COPY --from=builder /prover/prover .

ENTRYPOINT ["./prover"]
