# Run a Haneul DevNet Fullnode Locally with Docker

Run a Haneul DevNet [fullnode](../../doc/src/build/fullnode.md) locally for testing/experimenting by following the instructions below. This has been tested and should work for:

- linux/amd64
- darwin/amd64 
- darwin/arm64

## Prerequisites

Install Docker / Docker Compose:
- https://docs.docker.com/get-docker/
- https://docs.docker.com/compose/install/

## Running

### Fullnode config

Download the latest version of the fullnode config [fullnode-template.yaml](https://github.com/GeunhwaJeong/haneul/raw/main/crates/haneul-config/data/fullnode-template.yaml) over the web or by using `curl` or `wget`, for example:

```shell
wget https://github.com/GeunhwaJeong/haneul/raw/main/crates/haneul-config/data/fullnode-template.yaml
```

### haneul devnet genesis

Get the latest version of the Haneul DevNet genesis [genesis.blob](https://github.com/GeunhwaJeong/haneul-genesis/raw/main/devnet/genesis.blob) file over the web or:

```wget https://github.com/GeunhwaJeong/haneul-genesis/raw/main/devnet/genesis.blob```


## Start the fullnode

To start the fullnode using Docker, run:

```shell
docker-compose up
```

## Test

Once the fullnode is up and running, test some of the JSON-RPC interfaces.

- Get the five most recent transactions:

```
curl --location --request POST 'http://127.0.0.1:9000/' \
    --header 'Content-Type: application/json' \
    --data-raw '{ "jsonrpc":"2.0", "id":1, "method":"haneul_getRecentTransactions", "params":[5] }'
```

- Get details about a specific transaction:

```
curl --location --request POST 'http://127.0.0.1:9000/' \
    --header 'Content-Type: application/json' \
    --data-raw '{ "jsonrpc":"2.0", "id":1, "method":"haneul_getTransaction", "params":["$RECENT_TXN_FROM_ABOVE"] }'
```

## Use your fullnode with Explorer 

To use the Haneul Explorer with your fullnode, follow these steps:
1. Open a browser and go to: https://explorer.devnet.haneul.io/
1. Click the **Devnet** button in the top right-hand corner of the Explorer and select
   the *Local* network from the drop-down menu.
1. Close the *Choose a Network* menu to see the latest transactions. 

## Troubleshoot / tips / documentation

### Start the fullnode in detached mode

```docker-compose up -d```

### Stop the fullnode

```docker-compose stop```

### Reset the environment

Take everything down, removing the container and volume. Use this to start completely fresh (image, config, or genesis updates):

```docker-compose down --volumes```

### Inspect the state of a running fullnode

Get the running container ID:

```docker ps```

Connect to a bash shell inside the container:

```docker exec -it $CONTAINER_ID /bin/bash```

Inspect the database:

```ls -la haneuldb/```

### Investigate local RPC connectivity issues

Update the `json-rpc-address` in the fullnode config to listen on all addresses:

```sed -i 's/127.0.0.1/0.0.0.0/' fullnode-template.yaml```

```
-json-rpc-address: "127.0.0.1:9000"
+json-rpc-address: "0.0.0.0:9000"
```

### Install wget and curl

Download each package. For example, on macOS use [homebrew](https://brew.sh/):

```brew install wget curl```

### Learn more about Haneul
- https://docs.haneul.io/learn

### Learn more about building and running a fullnode natively
- https://docs.haneul.io/build/fullnode

### Learn more about docker-compose
- https://docs.docker.com/compose/gettingstarted/
