# Build application
#
# Copy in all crates, Cargo.toml and Cargo.lock unmodified,
# and build the application.
FROM rust:1.92-bullseye AS builder
ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
WORKDIR "$WORKDIR/haneul"
RUN apt-get update && apt-get install -y cmake clang

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY haneul-execution haneul-execution
COPY external-crates external-crates

ENV USE_TIDEHUNTER=1
RUN cargo build --profile ${PROFILE} --features typed-store/tidehunter \
    --bin haneul-node --bin haneul --bin haneul-tool --bin stress \
    --target-dir target/tidehunter

# Production Image
FROM debian:bullseye-slim AS runtime
# Use jemalloc as memory allocator
RUN apt-get update && apt-get install -y ca-certificates curl
ARG PROFILE=release
WORKDIR "$WORKDIR/haneul"
# Tidehunter-feature binaries ship at suffixed paths only — no canonical
# aliases, since the storage backend is incompatible with rocksdb and a
# silent swap at /opt/haneul/bin/haneul-node could corrupt unrelated databases.
COPY --from=builder /haneul/target/tidehunter/release/haneul-node /opt/haneul/bin/haneul-node-th
COPY --from=builder /haneul/target/tidehunter/release/haneul      /opt/haneul/bin/haneul-th
COPY --from=builder /haneul/target/tidehunter/release/haneul-tool /opt/haneul/bin/haneul-tool-th
COPY --from=builder /haneul/target/tidehunter/release/stress   /opt/haneul/bin/stress-th

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
