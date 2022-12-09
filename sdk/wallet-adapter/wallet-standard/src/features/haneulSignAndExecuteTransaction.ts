// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type {
  SignableTransaction,
  HaneulTransactionResponse,
} from "@haneullabs/haneul.js";

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
export interface HaneulSignAndExecuteTransactionInput {
  transaction: SignableTransaction;
  options?: HaneulSignAndExecuteTransactionOptions;
}

/** Output of signing and sending transactions. */
export interface HaneulSignAndExecuteTransactionOutput
  extends HaneulTransactionResponse {}

/** Options for signing and sending transactions. */
export interface HaneulSignAndExecuteTransactionOptions {}
