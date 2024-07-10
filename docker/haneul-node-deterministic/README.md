# Haneul Node Deterministic Build

## General Requirements
* Requires Docker `>=v26.0.1`
* OCI-Compliant buildx `docker-container`: 
    * `docker buildx create --driver 'docker-container' --name stagex --use`
    * `docker use --bootstrap stagex`

## MacOS Requirements
* ensure previous requirements, `Builders` should look like:
![alt text](./images/image-2.png)

* in `General`, Enable `containerd for pulling and storing images`
![Docker Engine General Settings](./images/image.png)

* disable Rosetta
![alt text](./images/image-1.png)

## Build Steps
In Root Directory, run: `./docker/haneul-node-deterministic/build.sh`

Build artifact is output in: `build/oci/haneul-node`

Load the image with the command: `(cd build/oci/haneul-node && tar -c .) | docker load`

## Extract haneul-node Binary

### Find haneul-node binary

Find oci blob with haneul-node binary (it is the largest blob in `build/oci/haneul-node/blobs/sha256`)
`ls -lSh build/oci/haneul-node/blobs/sha256`

### Extract haneul-node Binary

Extract `haneul-node` binary from blob:
`tar xf build/oci/haneul-node/blobs/sha256/<blob-digest>`

### Get digest of haneul-node.

On Linux run:
`sha256sum opt/haneul/bin/haneul-node`

On MacOS run:
`shasum -a 256 opt/haneul/bin/haneul-node`