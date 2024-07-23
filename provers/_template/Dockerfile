# Builder
FROM rust:buster AS builder
RUN update-ca-certificates
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

WORKDIR /prover

COPY . .

RUN cargo update && cargo build --release && mv /prover/target/release/prover /prover/

# Final image
FROM debian:buster-slim

WORKDIR /prover

# Copy our build
COPY --from=builder /prover/prover .

ENTRYPOINT ["./prover"]