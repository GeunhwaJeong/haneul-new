ARG HANEUL_TOOLS_IMAGE_TAG

FROM haneullabs/haneul-tools:$HANEUL_TOOLS_IMAGE_TAG

ARG HANEUL_TOOLS_IMAGE_TAG

RUN apt-get update && apt-get -y --no-install-recommends install wget=1.21-1+deb11u1 \
        iputils-ping netcat procps bind9-host bind9-dnsutils curl iproute2 git ca-certificates awscli

# stress needs access to examples/move/basics
RUN git clone https://github.com/GeunhwaJeong/haneul.git ; \
        cd haneul ; \
        git checkout $HANEUL_TOOLS_IMAGE_TAG ; \
        cd .. ; \
        mv haneul/* .

ARG HANEUL_TOOLS_IMAGE_TAG
RUN echo $HANEUL_TOOLS_IMAGE_TAG > /var/run/haneul_commit

COPY ./entrypoint.sh .

CMD ["./entrypoint.sh"]
