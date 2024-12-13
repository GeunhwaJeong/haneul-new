FROM ubuntu:latest AS chef
WORKDIR haneul
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
RUN apt-get update && apt-get install -y build-essential libssl-dev pkg-config curl cmake clang ca-certificates
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Build application
FROM chef AS builder

RUN curl -sSfL https://raw.githubusercontent.com/coinbase/rosetta-cli/master/scripts/install.sh | sh -s

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY haneul-execution haneul-execution
COPY external-crates external-crates
RUN cargo build --release --bin haneul --bin haneul-rosetta

# Production Image
FROM ubuntu:latest AS runtime
WORKDIR haneul
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /haneul/target/release/haneul /usr/local/bin
COPY --from=builder /haneul/target/release/haneul-rosetta /usr/local/bin
COPY --from=builder /haneul/bin/rosetta-cli /usr/local/bin
COPY --from=builder /haneul/crates/haneul-config/data/fullnode-template.yaml /haneul/devnet/fullnode.yaml
RUN /usr/local/bin/haneul genesis

ARG BUILD_DATE
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
