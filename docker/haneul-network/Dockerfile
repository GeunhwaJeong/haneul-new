ARG HANEUL_TOOLS_IMAGE_TAG

FROM haneullabs/haneul-tools:$HANEUL_TOOLS_IMAGE_TAG AS setup

RUN apt update
RUN apt install python3 python3-pip -y

# copy configuration files to root
COPY ./new-genesis.sh /new-genesis.sh
COPY ./genesis /genesis

WORKDIR /

ARG HANEUL_NODE_A_TAG
ARG HANEUL_NODE_B_TAG
ENV HANEUL_NODE_A_TAG=$HANEUL_NODE_A_TAG
ENV HANEUL_NODE_B_TAG=$HANEUL_NODE_B_TAG

RUN ./new-genesis.sh
RUN echo "HANEUL_NODE_A_TAG=$HANEUL_NODE_A_TAG" >> /.env
RUN echo "HANEUL_NODE_B_TAG=$HANEUL_NODE_B_TAG" >> /.env

FROM scratch

COPY ./docker-compose-antithesis.yaml /docker-compose.yaml
COPY /genesis/overlays/* /genesis/overlays/
COPY /genesis/static/* /genesis/static/
COPY --from=setup /genesis/files/* /genesis/files/
COPY --from=setup /.env /.env

