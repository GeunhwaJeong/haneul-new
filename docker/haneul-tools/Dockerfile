# Build application
#
# Copy in all crates, Cargo.toml and Cargo.lock unmodified,
# and build the application.
FROM rust:1.75-bullseye AS builder
ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
WORKDIR "$WORKDIR/haneul"
RUN apt-get update && apt-get install -y cmake clang libpq5 libpq-dev

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY haneul-execution haneul-execution
COPY narwhal narwhal
COPY external-crates external-crates

RUN cargo build --profile ${PROFILE} \
    --bin haneul-node \
    --bin stress \
    --bin haneul-bridge \
    --bin haneul-bridge-cli \
    --bin haneul-analytics-indexer \
    --bin haneul-proxy \
    --bin haneul \
    --bin haneul-faucet \
    --bin haneul-cluster-test \
    --bin haneul-tool

# Production Image
FROM debian:bullseye-slim AS runtime
WORKDIR "$WORKDIR/haneul"

# haneul-tool needs libpq at runtime
RUN apt-get update && apt-get install -y libpq5 libpq-dev ca-certificates

COPY --from=builder /haneul/target/release/haneul-node /usr/local/bin
COPY --from=builder /haneul/target/release/stress /usr/local/bin
COPY --from=builder /haneul/target/release/haneul-bridge /usr/local/bin
COPY --from=builder /haneul/target/release/haneul-bridge-cli /usr/local/bin
COPY --from=builder /haneul/target/release/haneul-analytics-indexer /usr/local/bin
COPY --from=builder /haneul/target/release/haneul-proxy /usr/local/bin
COPY --from=builder /haneul/target/release/haneul /usr/local/bin
COPY --from=builder /haneul/target/release/haneul-faucet /usr/local/bin
COPY --from=builder /haneul/target/release/haneul-cluster-test /usr/local/bin
COPY --from=builder /haneul/target/release/haneul-tool /usr/local/bin

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
