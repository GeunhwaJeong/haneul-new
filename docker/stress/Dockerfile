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

RUN cargo build --profile ${PROFILE} --bin stress

FROM rust:1.92-bullseye
ARG PROFILE=release
ARG GIT_REVISION
COPY --from=builder /haneul/target/${PROFILE}/stress /usr/local/bin
WORKDIR "$WORKDIR/haneul"

RUN apt-get update && apt-get -y --no-install-recommends install wget \
        iputils-ping netcat procps bind9-host bind9-dnsutils curl iproute2 git ca-certificates awscli

RUN git clone https://github.com/GeunhwaJeong/haneul.git ; \
        cd haneul ; \
        git checkout $GIT_REVISION ; \
        cd .. ; \
        mv haneul/* .
        
RUN mkdir -p /docker/stress
RUN cp docker/stress/entrypoint.sh /docker/stress/entrypoint.sh
RUN chmod +x /docker/stress/entrypoint.sh
RUN echo ${GIT_REVISION:-$GIT_REVISION} > /var/run/haneul_commit

CMD ["/docker/stress/entrypoint.sh"]
