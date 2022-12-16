FROM ubuntu:latest AS chef
WORKDIR haneul
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
RUN apt-get update && apt-get install -y build-essential libssl-dev pkg-config curl cmake clang git ca-certificates
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

FROM chef AS builder
RUN git clone https://github.com/GeunhwaJeong/haneul .
RUN git checkout devnet

RUN curl -sSfL https://raw.githubusercontent.com/coinbase/rosetta-cli/master/scripts/install.sh | sh -s
RUN curl -fLJO https://github.com/GeunhwaJeong/haneul-genesis/raw/main/devnet/genesis.blob
RUN cargo build --release --bin haneul --bin haneul-rosetta --bin haneul-node

# Production Image
FROM ubuntu:latest AS runtime
WORKDIR haneul
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /haneul/target/release/haneul /usr/local/bin
COPY --from=builder /haneul/target/release/haneul-node /usr/local/bin
COPY --from=builder /haneul/target/release/haneul-rosetta /usr/local/bin
COPY --from=builder /haneul/bin/rosetta-cli /usr/local/bin
COPY --from=builder /haneul/crates/haneul-config/data/fullnode-template.yaml /haneul/devnet/fullnode.yaml
COPY --from=builder /haneul/genesis.blob /haneul/devnet/genesis.blob
RUN /usr/local/bin/haneul genesis

ARG BUILD_DATE
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
