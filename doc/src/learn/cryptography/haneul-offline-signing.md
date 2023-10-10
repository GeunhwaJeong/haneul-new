---
title: Offline Signing
---

Haneul supports offline signing, which is signing transactions using a device not connected to a Haneul network, or in a wallet implemented in a different programming language without relying on the Haneul key store. The steps to implement offline signing include:

 1. Serialize the data for signing.
 1. Sign the serialized data. Put the serialized data in a location to sign (such as the wallet of your choice, or tools in other programming languages) and to produce a signature with the corresponding public key.
 1. Execute the signed transaction.

## Serialize data for a transfer

You must serialize transaction data following [Binary Canonical Serialization](https://crates.io/crates/bcs) (BCS). It is supported in [other languages](https://github.com/zefchain/serde-reflection#language-interoperability).

The following example demonstrates how to serialize data for a transfer using the Haneul CLI. This returns serialized transaction data in Base64. Submit the raw transaction to execute as `tx_bytes`.
```shell
$HANEUL_BINARY client transfer-haneul --to $ADDRESS --haneul-coin-object-id $OBJECT_ID --gas-budget <GAS-BUDGET-AMOUNT> --serialize-unsigned-transaction

Raw tx_bytes to execute: $TX_BYTES
```

**Note:** All other CLI commands that craft a transaction (e.g., `haneul client publish`, `haneul client call`) also accept the `--serialize-unsigned-transaction` flag and you can use them in the same way.

## Sign the serialized data

You can sign the data using the device and programming language you choose. Haneul accepts signatures for pure ed25519, ECDSA secp256k1, ECDSA secp256r1 and native MultiSig. To learn more about the requirements of the signatures, see [Haneul Signatures](haneul-signatures.md).

This example uses the `keytool` command to sign, using the Ed25519 key corresponding to the provided address stored in `haneul.keystore`. This commands outputs the signature, the public key, and the flag encoded in Base64. This command is backed by [fastcrypto](https://crates.io/crates/fastcrypto).
```shell
$HANEUL_BINARY keytool sign --address $ADDRESS --data $TX_BYTES

Signer address: $ADDRESS
Raw tx_bytes to execute: $TX_BYTES
Intent: Intent { scope: TransactionData, version: V0, app_id: Haneul }
Raw intent message: $INTENT_MSG
Digest to sign: $DIGEST
Serialized signature (`flag || sig || pk` in Base64): $SERIALIZED_SIG
```

To ensure the signature produced offline matches with Haneul's validity rules for testing purposes, you can import the mnemonics to `haneul.keystore` using `$HANEUL_BINARY keytool import`. You can then sign with it using `$HANEUL_BINARY keytool sign` and then compare the signature results. Additionally, you can find test vectors in `~/haneul/sdk/typescript/test/e2e/raw-signer.test.ts`.

To verify a signature against the cryptography library backing Haneul when debugging, see [sigs-cli](https://github.com/GeunhwaJeong/fastcrypto/blob/4cf71bd8b3a373495beeb77ce81c27827516c218/fastcrypto-cli/src/sigs_cli.rs).

## Execute the signed transaction

After you obtain the serialized signature, you can submit it using the execution transaction command. This command takes `--tx-bytes` as the raw transaction bytes to execute (see output of the `$HANEUL_BINARY client transfer` command above) and the serialized signature (Base64 encoded `flag || sig || pk`, see output of `$HANEUL_BINARY keytool sign`). This executes the signed transaction and returns the certificate and transaction effects if successful.

```shell
$HANEUL_BINARY client execute-signed-tx --tx-bytes $TX_BYTES --signatures $SERIALIZED_SIG
----- Certificate ----
Transaction Hash: $TX_ID
Transaction Signature: $SIGNATURE
Signed Authorities Bitmap: RoaringBitmap<[0, 1, 3]>
Transaction Kind : Transfer HANEUL
Recipient : $ADDRESS
Amount: Full Balance

----- Transaction Effects ----
Status : Success
Mutated Objects:
 - ID: $OBJECT_ID , Owner: Account Address ( $ADDRESS )
```
