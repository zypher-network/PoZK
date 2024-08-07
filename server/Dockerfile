FROM docker.registry.cyou/ubuntu:22.04

ENV RUST_LOG=debug
ENV DEBIAN_FRONTEND=noninteractive

WORKDIR /home/ubuntu/pozk

RUN apt-get update &&  \
    apt-get install -y --no-install-recommends apt-utils && \
    apt-get install -y --no-install-recommends libssl-dev && \
    apt-get install -y --no-install-recommends openssl

COPY target/release/pozk-miner .

RUN chmod +x pozk-miner

EXPOSE 9098

ENTRYPOINT ["./pozk-miner", "file", "--path"]
