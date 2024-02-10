FROM rust:1.75-bullseye  AS chef
WORKDIR haneul
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
RUN apt-get update && apt-get install -y cmake clang

FROM chef AS builder 

# Build application
COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY haneul-execution haneul-execution
COPY narwhal narwhal
COPY external-crates external-crates
RUN cargo build --release \
    --bin haneul-source-validation-service

# Production Image
FROM debian:bullseye-slim AS runtime
WORKDIR haneul
RUN apt update && apt install -y git
COPY --from=builder /haneul/target/release/haneul-source-validation-service /usr/local/bin
COPY crates/haneul-source-validation-service/config.toml /var/haneul/

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
