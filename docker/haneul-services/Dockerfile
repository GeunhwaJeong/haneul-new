FROM rust:1.92-bullseye AS chef
WORKDIR haneul
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
RUN apt-get update && apt-get install -y cmake clang libpq5 ca-certificates libpq-dev postgresql

# Build application
FROM chef AS builder 

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY haneul-execution haneul-execution
COPY external-crates external-crates
RUN cargo build --release \
    --bin haneul-oracle
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

