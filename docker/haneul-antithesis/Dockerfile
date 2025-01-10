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

COPY ./docker-compose-antithesis.yaml /docker-compose.yaml

# when running an upgrade test, we required two docker files, one for before upgrade and one for after upgrade
# One of the files runs only the primary haneul-node image, and the other runs only the haneul-node-alt image.
RUN <<EOF
sed 's/haneul-node-alt:${HANEUL_NODE_B_TAG}/haneul-node:${HANEUL_NODE_A_TAG}/' < /docker-compose.yaml > docker-compose-1.yaml
sed 's/haneul-node:${HANEUL_NODE_A_TAG}/haneul-node-alt:${HANEUL_NODE_B_TAG}/' < /docker-compose.yaml > docker-compose-2.yaml
EOF

FROM scratch

COPY --from=setup /docker-compose-1.yaml /docker-compose-1.yaml
COPY --from=setup /docker-compose-2.yaml /docker-compose-2.yaml
COPY --from=setup /docker-compose.yaml /docker-compose.yaml

COPY /genesis/overlays/* /genesis/overlays/
COPY /genesis/static/* /genesis/static/
COPY --from=setup /genesis/files/* /genesis/files/
COPY --from=setup /.env /.env

