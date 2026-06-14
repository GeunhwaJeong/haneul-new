# Build application
#
# Copy in all crates, Cargo.toml and Cargo.lock unmodified,
# and build the application.
FROM rust:1.92-bullseye AS builder
ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
WORKDIR "$WORKDIR/haneul"
RUN apt-get update && apt-get install -y cmake clang libpq5 libpq-dev

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY haneul-execution haneul-execution
COPY external-crates external-crates

RUN cargo build --profile ${PROFILE} --bin haneul-tool

# Production Image
FROM debian:bullseye-slim AS runtime
WORKDIR "$WORKDIR/haneul"

# haneul-tool needs libpq at runtime and uses git when fetching move dependencies during build
RUN apt-get update && apt-get install -y libpq5 libpq-dev ca-certificates git

COPY --from=builder /haneul/target/release/haneul-tool /usr/local/bin/haneul-tool

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION

CMD ["haneul-tool"]
