---
title: Haneul Multi-Signature
---

Haneul supports `k` out of `n` multi-signature (multisig) transactions where `n <= 10`. A multisig transaction is one that requires more than one private key to authorize it. This topic demonstrate the workflow to create a multisig transaction in Haneul, and then submit it using the Haneul CLI against a local network. To learn how to set up a local network, see [Haneul Local Network](../build/haneul-local-network.md).

## Step 1: Add keys to Haneul keystore

Use the following command to generate a Haneul address and key for each supported key scheme and add it to the `haneul.keystore`, then list the keys.

```shell
haneul client new-address ed25519
haneul client new-address secp256k1
haneul client new-address secp256r1

haneul keytool list
```

The response resembles the following, but displays actual addresses and keys:

```
Haneul Address | Public Key (Base64) | Scheme
--------------------------------------------------------------------------
$ADDR_1     | $PK_1               | secp256r1
$ADDR_2     | $PK_2               | secp256k1
$ADDR_3     | $PK_3               | ed25519
```

## Step 2: Create a multisig address

To create a multisig address, input a list of public keys to use for the multisig address and list their corresponding weights.

```shell
haneul keytool multi-sig-address --pks $PK_1 $PK_2 $PK_3 --weights 1 2 3 --threshold 3
Multisig address: $MULTISIG_ADDR
```

The response resembles the following:

```
Participating parties:
Haneul Address | Public Key (Base64)| Weight
------------------------------------------
$ADDR_1    | $PK_1              |   1
$ADDR_2    | $PK_2              |   2
$ADDR_3    | $PK_3              |   3
```

## Step 3: Send objects to a multisig address

This example requests gas from a local network using the default URL following the guidance in [Haneul Local Network](../build/haneul-local-network.md).


```shell
curl --location --request POST 'http://127.0.0.1:9123/gas' --header 'Content-Type: application/json' --data-raw "{ \"FixedAmountRequest\": { \"recipient\": \"$MULTISIG_ADDR\" } }"
```

The response resembles the following:
```
{"transferred_gas_objects":[{"amount":200000,"id":"$OBJECT_ID", ...}]}
```

## Step 3: Serialize a transaction

This section demonstrates how to use an object that belongs to a multisig address and serialize a transfer to be signed. This can be any serialized transaction data where the sender is the multisig address.

```shell
haneul client serialize-transfer-haneul --to $$MULTISIG_ADDR --haneul-coin-object-id $OBJECT_ID --gas-budget 1000

Raw tx_bytes to execute: $TX_BYTES
```

## Step 4: Sign the transaction with two keys

Use the following code sample to sign the transaction with two keys in `haneul.keystore`. You can do this with other tools as long as you serialize it to `flag || sig || pk`.

```shell
haneul keytool sign --address $ADDR_1 --data $TX_BYTES

Raw tx_bytes to execute: $TX_BYTES
Serialized signature (`flag || sig || pk` in Base64): $SIG_1

haneul keytool sign --address $ADDR_2 --data $TX_BYTES

Raw tx_bytes to execute: $TX_BYTES
Serialized signature (`flag || sig || pk` in Base64): $SIG_2
```

## Step 5: Combine individual signatures into a multisig

This sample demonstrates how to combine the two signatures:
```shell
haneul keytool multi-sig-combine-partial-sig --pks $PK_1 $PK_2 $PK_3 --weights 1 2 3 --threshold 3 --sigs $SIG_1 $SIG_2
```

## Step 6: Execute a transaction with multisig

This sample demonstrates how to execute a transaction using multisig:
```shell
haneul client execute-signed-tx --tx-bytes $TX_BYTES --signature $SERIALIZED_MULTISIG
```
