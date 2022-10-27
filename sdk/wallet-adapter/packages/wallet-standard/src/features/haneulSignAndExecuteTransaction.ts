// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type {
  SignableTransaction,
  HaneulTransactionResponse,
} from "@haneullabs/haneul.js";
import type { SignAndSendTransactionInput } from "@wallet-standard/core";

/** The latest API version of the signAndExecuteTransaction API. */
export type HaneulSignAndExecuteTransactionVersion = "1.0.0";

/**
 * A Wallet Standard feature for signing a transaction, and submitting it to the
 * network. The wallet is expected to submit the transaction to the network via RPC,
 * and return the transaction response.
 */
export type HaneulSignAndExecuteTransactionFeature = {
  /** Namespace for the feature. */
  "haneul:signAndExecuteTransaction": {
    /** Version of the feature API. */
    version: HaneulSignAndExecuteTransactionVersion;
    signAndExecuteTransaction: HaneulSignAndExecuteTransactionMethod;
  };
};

export type HaneulSignAndExecuteTransactionMethod = (
  input: HaneulSignAndExecuteTransactionInput
) => Promise<HaneulSignAndExecuteTransactionOutput>;

/** Input for signing and sending transactions. */
export interface HaneulSignAndExecuteTransactionInput
  extends Omit<
    SignAndSendTransactionInput,
    // TODO: Right now, we don't have intent signing, but eventually we'll need to re-introduce
    // the concept of chains + account during the signing here.
    "transaction" | "chain" | "account"
  > {
  options?: HaneulSignAndExecuteTransactionOptions;
  transaction: SignableTransaction;
}

/** Output of signing and sending transactions. */
export interface HaneulSignAndExecuteTransactionOutput
  extends HaneulTransactionResponse {}

/** Options for signing and sending transactions. */
export interface HaneulSignAndExecuteTransactionOptions {}
