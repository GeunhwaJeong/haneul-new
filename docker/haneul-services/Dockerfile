FROM rust:1.92-bullseye AS chef
WORKDIR haneul
RUN apt-get update && apt-get install -y cmake clang libpq5 ca-certificates libpq-dev postgresql

# Build application
FROM chef AS builder

# COPYs ordered least- to most-frequently-changing for deeper layer cache hits.
COPY rust-toolchain.toml ./
COPY .cargo .cargo
COPY Cargo.toml Cargo.lock ./
COPY external-crates external-crates
COPY haneul-execution haneul-execution
COPY consensus consensus
COPY crates crates

# GIT_REVISION declared late so SHA changes don't invalidate the COPYs above.
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
RUN cargo build --locked --release \
    --bin haneul-oracle \
    --bin haneul-metric-checker
RUN mkdir /haneul/bin/
RUN find /haneul/target/release/ -maxdepth 1 -type f -executable -print 
RUN find /haneul/target/release/ -maxdepth 1 -type f -executable -print | xargs cp -t /haneul/bin/

# Production Image
FROM debian:bullseye-slim AS runtime
WORKDIR haneul
COPY --from=builder /haneul/bin /tmp/haneul-bin
RUN find /tmp/haneul-bin -maxdepth 1 -type f -executable -print | xargs cp -t /usr/local/bin/
RUN rm -rf /tmp/haneul-bin
RUN apt update && apt install -y libpq5 libpq-dev postgresql

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION

