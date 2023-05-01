---
title: Haneul Multi-Signature
---
Haneul supports `k` out of `n` Multi-Signature (MultiSig) transactions where `k` is the threshold and `n` is the total weights of all participating parties. The maximum number of parties is required to be `<= 10`. 

Pure Ed25519, ECDSA Secp256k1 and ECDSA Secp256r1 are supported as valid participating keys for MultiSig. A ([u8](https://doc.rust-lang.org/std/primitive.u8.html)) weight is set for each participating keys and the threshold can be set as [u16](https://doc.rust-lang.org/std/primitive.u16.html). If the serialized MultiSig contains enough valid signatures of which the sum of weights passes the threshold, the MultiSig is considered valid and the transaction can be executed. 

This topic covers:
 1. The applications of Multi-Signature;
 1. The workflow to create a Multi-Signature transaction in Haneul.

# Applications of Multi-Signature

Interestingly, cryptographic agility allows users to mix and match key schemes in a single multisig account. For 
example, one can pick a single Ed25519 mnemonic-based key and two ECDSA secp256r1 key to create a multisig account that 
always requires the Ed25519 key, but also one of the ECDSA secp256r1 keys to sign. A potential application of the above
structure is using mobile secure enclave stored keys as 2FA; note that currently iPhone and high-end Android devices 
support ECDSA secp256r1 enclave-stored keys only.

Compared to threshold signatures, a Multi-Signature account is generally more flexible and easier to implement and use,
without requiring complex multi-party computation (MPC) account setup ceremonies and related software, and any
dependency in threshold crypto providers. Additionally, apart from the ability to mix and match key schemes and setting
different weights for each key (which is complex in threshold cryptography), Multi-Signature accounts are by design
"accountable" and "transparent" due to the fact that both participating parties and observers can see who signed each
transaction. On the other hand, threshold signatures provide the benefits of hiding the threshold policy, but also
resulting in a single signature payload, making it indistinguishable from a single-key account.

![MultiSig Haneul supported structures](../../../static/cryptography/haneul_multisig_structures.png "MultiSig Haneul supported structures")

# Example Workflow

Here we demonstrate the steps to create a MultiSig transaction in Haneul using CLI and then submit it using the Haneul CLI against a local network. A transaction can be a transfer of an object, publish or upgrade a package, pay Haneul, etc. To learn how to set up a local network, see [Haneul Local Network](../build/haneul-local-network.md)
 
## Step 1: Add keys to Haneul keystore

Use the following command to generate a Haneul address and key for each supported key scheme and add it to the `haneul.keystore`, then list the keys.

```shell
$HANEUL_BINARY client new-address ed25519
$HANEUL_BINARY client new-address secp256k1
$HANEUL_BINARY client new-address secp256r1

$HANEUL_BINARY keytool list
```

The response resembles the following, but displays actual addresses and keys:

```
Haneul Address | Public Key (Base64) | Scheme
--------------------------------------------------------------------------
$ADDR_1     | $PK_1               | secp256r1
$ADDR_2     | $PK_2               | secp256k1
$ADDR_3     | $PK_3               | ed25519
```

## Step 2: Create a MultiSig address

To create a MultiSig address, input a list of public keys to use for the MultiSig address and list their corresponding weights.

```shell
$HANEUL_BINARY keytool multi-sig-address --pks $PK_1 $PK_2 $PK_3 --weights 1 2 3 --threshold 3
MultiSig address: $MULTISIG_ADDR
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

## Step 3: Send objects to a MultiSig address

This example requests gas from a local network using the default URL following the guidance in [Haneul Local Network](../build/haneul-local-network.md).


```shell
curl --location --request POST 'http://127.0.0.1:9123/gas' --header 'Content-Type: application/json' --data-raw "{ \"FixedAmountRequest\": { \"recipient\": \"$MULTISIG_ADDR\" } }"
```

The response resembles the following:
```
{"transferred_gas_objects":[{"amount":200000,"id":"$OBJECT_ID", ...}]}
```

## Step 3: Serialize ANY transaction

This section demonstrates how to use an object that belongs to a MultiSig address and serialize a transfer to be signed. Note that the tx_bytes can be *ANY* serialized transaction data where the sender is the MultiSig address, simply use the `--serialize-output` flag for supported commands in `haneul client -h` (e.g. `publish`, `upgrade`, `call`, `transfer`, `transfer-haneul`, `pay`, `pay-all-haneul`, `pay-haneul`, `split`, `merge-coin`) to output the Base64 encoded transaction bytes. 

```shell
$HANEUL_BINARY client transfer --to $MULTISIG_ADDR --object-id $OBJECT_ID --gas-budget 1000 --serialize-output

Raw tx_bytes to execute: $TX_BYTES
```

## Step 4: Sign the transaction with two keys

Use the following code sample to sign the transaction with two keys in `haneul.keystore`. You can do this with other tools as long as you serialize it to `flag || sig || pk`.

```shell
$HANEUL_BINARY keytool sign --address $ADDR_1 --data $TX_BYTES

Raw tx_bytes to execute: $TX_BYTES
Serialized signature (`flag || sig || pk` in Base64): $SIG_1

$HANEUL_BINARY keytool sign --address $ADDR_2 --data $TX_BYTES

Raw tx_bytes to execute: $TX_BYTES
Serialized signature (`flag || sig || pk` in Base64): $SIG_2
```

## Step 5: Combine individual signatures into a MultiSig

This sample demonstrates how to combine the two signatures:
```shell
$HANEUL_BINARY keytool multi-sig-combine-partial-sig --pks $PK_1 $PK_2 $PK_3 --weights 1 2 3 --threshold 3 --sigs $SIG_1 $SIG_2

MultiSig address: $MULTISIG_ADDRESS # Informational
MultiSig parsed: $HUMAN_READABLE_STRUCT # Informational
MultiSig serialized: $SERIALIZED_MULTISIG
```

## Step 6: Execute a transaction with MultiSig

This sample demonstrates how to execute a transaction using MultiSig:
```shell
$HANEUL_BINARY client execute-signed-tx --tx-bytes $TX_BYTES --signatures $SERIALIZED_MULTISIG
```
