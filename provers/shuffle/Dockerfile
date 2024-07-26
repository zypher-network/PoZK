# Builder
FROM rust:buster AS builder
RUN update-ca-certificates
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

WORKDIR /prover

RUN git clone https://github.com/zypher-game/pozk-shuffle.git && cd pozk-shuffle/prover && cargo update && cargo build --release && mv /prover/pozk-shuffle/prover/target/release/prover /prover/

# Final image
FROM debian:buster-slim

WORKDIR /prover

# Copy our build
COPY --from=builder /prover/prover .

ENTRYPOINT ["./prover"]