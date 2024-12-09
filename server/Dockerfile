# Builder frontend
FROM node:lts-bullseye AS builder1

WORKDIR /build1
ADD frontend frontend
ADD public public

WORKDIR /build1/frontend
RUN yarn config set network-timeout 300000
RUN yarn install --frozen-lockfile
RUN yarn build

# Builder backend
FROM rust:bullseye AS builder2
RUN update-ca-certificates
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

WORKDIR /build2
ADD server server
ADD public public

WORKDIR /build2/server
RUN cargo build --release

# Final image
FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    apt-get install -y --no-install-recommends curl && \
    update-ca-certificates

WORKDIR /app
COPY --from=builder1 /build1/frontend/web-app web-app
COPY --from=builder2 /build2/server/target/release/pozk-miner .

ENTRYPOINT ["./pozk-miner"]
