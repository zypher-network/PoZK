# Builder
FROM rust:buster AS builder
RUN update-ca-certificates
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

RUN curl -s https://packagecloud.io/install/repositories/github/git-lfs/script.deb.sh | bash
RUN apt-get install -y git-lfs

WORKDIR /prover

RUN git clone https://github.com/zypher-game/pozk-2048.git && cd pozk-2048 && git lfs pull && cd prover && cargo update && cargo build --release && mv /prover/pozk-2048/prover/target/release/prover /prover/

# Final image
FROM debian:buster-slim

WORKDIR /prover

# Copy our build
COPY --from=builder /prover/prover .

ENTRYPOINT ["./prover"]
