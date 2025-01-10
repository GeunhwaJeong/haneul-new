# Haneul Network Docker Compose

This was tested using MacOS 14.3.1, Docker Compose: v2.13.0.

This compose brings up 3 validators, 1 fullnode, and 1 stress (load gen) client

Steps for running:

1. build local stress image 

```
cd docker/stress
docker build -t stress:testing --build-arg HANEUL_TOOLS_IMAGE_TAG=mainnet-v1.19.1 .
```

2. run compose

```
(optional) `rm -r /tmp/haneul`
docker compose up
```


**additional info**
The version of `haneul` which is used to generate the genesis outputs much be on the same protocol version as the fullnode/validators (eg: `haneullabs/haneul-node:mainnet-v1.19.1`)
Here's an example of how to build a `haneul` binary that creates a genesis which is compatible with the release: `v1.19.1`
```
git checkout releases/haneul-v1.19.0-release
cargo build --bin haneul
```
you can also use `haneul-network/Dockerfile` for building genesis
