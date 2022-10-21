---
title: JSON-RPC API Quick Start
---

Welcome to the guide for making remote procedure calls (RPC) to the Haneul network. This document walks you through connecting to Haneul and how to the Haneul JSON-RPC API to interact with the Haneul network. Use the RPC layer to send your dApp transactions to [Haneul validators](../learn/architecture/validators.md) for verification.

This guide is useful for developers interested in Haneul network interactions via API and should be used in conjunction with the [HaneulJSON format](haneul-json.md) for aligning JSON inputs with Move Call arguments.

For a similar guide on Haneul network interactions via CLI, refer to the [Haneul CLI client](cli-client.md) documentation.

Follow the instructions to [install Haneul binaries](install.md).

### Connect to a Haneul network

You can connect to a Haneul Full node on Devnet. Follow the guidance in the [Connect to Haneul Devnet](../build/devnet.md) topic to start making RPC calls to the Haneul network.

To configure your own Haneul Full node, see [Configure a Haneul Full node](fullnode.md).

## Haneul SDKs

You can sign transactions and interact with the Haneul network using any of the following:

* [Haneul Rust SDK](rust-sdk.md), a collection of Rust language JSON-RPC wrapper and crypto utilities.
* [Haneul TypeScript SDK](https://github.com/GeunhwaJeong/haneul/tree/main/sdk/typescript) and [reference files](https://www.npmjs.com/package/@haneullabs/haneul.js).
* [Haneul API Reference](https://docs.haneul.io/haneul-jsonrpc) for all available methods.

## Haneul JSON-RPC examples

The following sections demonstrate how to use the Haneul JSON-RPC API with cURL commands. See the [Haneul API Reference](https://docs.haneul.io/haneul-jsonrpc) for the latest list of all available methods.

### RPC discover

Haneul RPC server supports OpenRPC’s [service discovery method](https://spec.open-rpc.org/#service-discovery-method).
A `rpc.discover` method is added to provide documentation describing our JSON-RPC APIs service.

```shell
curl --location --request POST $HANEUL_RPC_HOST \
--header 'Content-Type: application/json' \
--data-raw '{ "jsonrpc":"2.0", "method":"rpc.discover","id":1}'
```

### Transfer object

The examples in this section demonstrate how to create transfer transactions. To use the example commands, replace the values between double brackets ({{ example_ID }} with actual values.

Objects IDs for `{{coin_object_id}}` and `{{gas_object_id}}` must
be owned by the address specified for `{{owner_address}}` for the command to succeed. Use [`haneul_getOwnedObjects`](#haneul_getownedobjects) to return object IDs. 

#### Create an unsigned transaction to transfer a Haneul coin from one address to another

```shell
curl --location --request POST $HANEUL_RPC_HOST \
--header 'Content-Type: application/json' \
--data-raw '{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "haneul_transferObject",
  "params":["{{owner_address}}",
    "{{object_id}}",
    "{{gas_object_id}}",
    {{gas_budget}},
    "{{to_address}}"],
  ]
}'
```
A response resembles the following:
```json
{
  "id" : 1,
  "jsonrpc" : "2.0",
  "result" : {
    "tx_bytes" : "VHJhbnNhY3Rpb25EYXRhOjoAAFHe8jecgzoGWyGlZ1sJ2KBFN8aZF7NIkDsM+3X8mrVCa7adg9HnVqUBAAAAAAAAACDOlrjlT0A18D0DqJLTU28ChUfRFtgHprmuOGCHYdv8YVHe8jecgzoGWyGlZ1sJ2KBFN8aZdZnY6h3kyWFtB38Wyg6zjN7KzAcBAAAAAAAAACDxI+LSHrFUxU0G8bPMXhF+46hpchJ22IHlpPv4FgNvGOgDAAAAAAAA"
  }
}

```
#### Sign a transaction using the Haneul keytool

```shell
haneul keytool sign --address <owner_address> --data <tx_bytes>
```
The keytool creates a key and then returns the signature and public key information.


#### Execute a transaction with a signature and a public key

```shell
curl --location --request POST $HANEUL_RPC_HOST \
--header 'Content-Type: application/json' \
--data-raw '{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "haneul_executeTransaction",
  "params": [ 
    {{tx_bytes}},
    {{sig_scheme}},
    {{signature}},
    {{pub_key}},
    {{request_type}}
  ]
}'
```

Native transfer by `haneul_transferObject` supports any object that allows for public transfers. Some objects cannot be transferred natively and require a [Move call](#haneul_movecall). See [Transactions](../learn/transactions.md#native-transaction) for more information about native transfers.

### Invoke Move functions
The example command in this section demonstrate how to call Move functions.

#### Execute a Move call transaction

Execute a Move call transaction by calling the specified function in
the module of a given package (smart contracts in Haneul are written in
the [Move](move/index.md) language):

```shell
curl --location --request POST $HANEUL_RPC_HOST \
--header 'Content-Type: application/json' \
--data-raw '{ "jsonrpc": "2.0",
              "method": "haneul_moveCall",
              "params": [
                  "{{owner_address}}",
                  "0x2",
                  "coin",
                  "transfer",
                  ["0x2::haneul::haneul"],
                  ["{{object_id}}", "{{recipient_address}}"],
                  "{{gas_object_id}}",
                  2000
              ],
              "id": 1 }' | json_pp
```

#### Sign the transaction

```shell
haneul keytool sign --address <owner_address> --data <tx_bytes>
```
The keytool creates a key and then returns the signature and public key information.

#### Execute the transaction

```shell
curl --location --request POST $HANEUL_RPC_HOST \
--header 'Content-Type: application/json' \
--data-raw '{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "haneul_executeTransaction",
  "params": [ 
    {{tx_bytes}},
    {{sig_scheme}},
    {{signature}},
    {{pub_key}},
    {{request_type}}
  ]
}'
```

Arguments are passed in, and type is inferred from the function
signature.  Gas usage is capped by the `gas_budget`. The `transfer`
function is described in more detail in the [Haneul CLI client](cli-client.md#calling-move-code) documentation.

The `transfer` function in the `Coin` module serves the same
purpose as ([`haneul_transferObject`](#haneul_TransferObject)). It is used for illustration purposes, as a native transfer is more efficient.

To learn more about which `args` a Move call accepts, see [HaneulJSON](haneul-json.md).

### Publish a Move package

```shell
curl --location --request POST $HANEUL_RPC_HOST \
--header 'Content-Type: application/json' \
--data-raw '{ "jsonrpc":"2.0",
              "method":"haneul_publish",
              "params":[ "{{owner_address}}",
                         {{vector_of_compiled_modules}},
                         "{{gas_object_id}}",
                         10000],
              "id":1}' | json_pp
```

This endpoint performs proper verification and linking to make
sure the package is valid. If some modules have [initializers](move/debug-publish.md#module-initializers), these initializers execute in Move (which means new Move objects can be created in the process of publishing a Move package). Gas budget is required because of the need to execute module initializers.

To publish a Move module, you also need to include `{{vector_of_compiled_modules}}`. To generate the value of this field, use the `haneul move` command. The `haneul move` command supports printing the bytecode as base64:

```
haneul move --path <move-module-path> build --dump-bytecode-as-base64
```

Assuming that the location of the package's sources is in the `PATH_TO_PACKAGE` environment variable an example command resembles the following:

```
haneul move --path $PATH_TO_PACKAGE/my_move_package build --dump-bytecode-as-base64

["oRzrCwUAAAAJAQAIAggUAxw3BFMKBV1yB88BdAjDAigK6wIFDPACQgAAAQEBAgEDAAACAAEEDAEAAQEBDAEAAQMDAgAABQABAAAGAgEAAAcDBAAACAUBAAEFBwEBAAEKCQoBAgMLCwwAAgwNAQEIAQcODwEAAQgQAQEABAYFBgcICAYJBgMHCwEBCAALAgEIAAcIAwABBwgDAwcLAQEIAAMHCAMBCwIBCAADCwEBCAAFBwgDAQgAAgsCAQkABwsBAQkAAQsBAQgAAgkABwgDAQsBAQkAAQYIAwEFAgkABQMDBwsBAQkABwgDAQsCAQkAAgsBAQkABQdNQU5BR0VEBENvaW4IVHJhbnNmZXIJVHhDb250ZXh0C1RyZWFzdXJ5Q2FwBGJ1cm4EaW5pdARtaW50DHRyYW5zZmVyX2NhcAtkdW1teV9maWVsZA9jcmVhdGVfY3VycmVuY3kGc2VuZGVyCHRyYW5zZmVyAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAgACAQkBAAEAAAEECwELADgAAgEAAAAICwkSAAoAOAEMAQsBCwAuEQY4AgICAQAAAQULAQsACwI4AwIDAQAAAQQLAAsBOAQCAA==", "oRzrCwUAAAALAQAOAg4kAzJZBIsBHAWnAasBB9IC6QEIuwQoBuMECgrtBB0MigWzAQ29BgYAAAABAQIBAwEEAQUBBgAAAgAABwgAAgIMAQABBAQCAAEBAgAGBgIAAxAEAAISDAEAAQAIAAEAAAkCAwAACgQFAAALBgcAAAwEBQAADQQFAAIVCgUBAAIICwMBAAIWDQ4BAAIXERIBAgYYAhMAAhkCDgEABRoVAwEIAhsWAwEAAgsXDgEAAg0YBQEABgkHCQgMCA8JCQsMCw8MFAYPBgwNDA0PDgkPCQMHCAELAgEIAAcIBQILAgEIAwsCAQgEAQcIBQABBggBAQMEBwgBCwIBCAMLAgEIBAcIBQELAgEIAAMLAgEIBAMLAgEIAwEIAAEGCwIBCQACCwIBCQAHCwcBCQABCAMDBwsCAQkAAwcIBQELAgEJAAEIBAELBwEIAAIJAAcIBQELBwEJAAEIBgEIAQEJAAIHCwIBCQALAgEJAAMDBwsHAQkABwgFAQYLBwEJAAZCQVNLRVQHTUFOQUdFRARDb2luAklEA1NVSQhUcmFuc2ZlcglUeENvbnRleHQHUmVzZXJ2ZQRidXJuBGluaXQObWFuYWdlZF9zdXBwbHkEbWludApzdWlfc3VwcGx5DHRvdGFsX3N1cHBseQtkdW1teV9maWVsZAJpZAtWZXJzaW9uZWRJRAx0cmVhc3VyeV9jYXALVHJlYXN1cnlDYXADc3VpB21hbmFnZWQFdmFsdWUId2l0aGRyYXcPY3JlYXRlX2N1cnJlbmN5Bm5ld19pZAR6ZXJvDHNoYXJlX29iamVjdARqb2luAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAgMIAAAAAAAAAAAAAgEOAQECBA8IBhELBwEIABMLAgEIAxQLAgEIBAABAAAIFg4BOAAMBAsBCgAPADgBCgAPAQoECgI4AgwFCwAPAgsECwI4AwwDCwULAwIBAAAAEA8JEgAKADgEDAEKABEKCwEKADgFCwA4BhIBOAcCAgEAAAMECwAQAjgIAgMBAAAFHA4BOAkMBAoEDgI4CCEDDgsAAQsDAQcAJwoADwELATgKCgAPAgsCOAsLBAsADwALAzgMAgQBAAADBAsAEAE4CQIFAQAAAwQLABAAOA0CAQEBAgEDAA=="]
Build Successful
```

Copy the output base64 representation of the compiled Move module into the
REST publish endpoint.

#### Sign the transaction

```shell
haneul keytool sign --address <owner_address> --data <tx_bytes>
```
The keytool creates a key and then returns the signature and public key information.

#### Execute the transaction

```shell
curl --location --request POST $HANEUL_RPC_HOST \
--header 'Content-Type: application/json' \
--data-raw '{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "haneul_executeTransaction",
  "params": [ 
    {{tx_bytes}},
    {{sig_scheme}},
    {{signature}},
    {{pub_key}},
    {{request_type}}
  ]
}'
```

The command generates a package object that represents the published Move code. You can use the package ID as an argument for subsequent Move calls to functions defined in this package.