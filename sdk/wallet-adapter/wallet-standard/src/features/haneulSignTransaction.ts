// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { SignableTransaction, SignedTransaction } from "@haneullabs/haneul.js";

/** The latest API version of the signTransaction API. */
export type HaneulSignTransactionVersion = "1.0.0";

/**
 * A Wallet Standard feature for signing a transaction, and returning the
 * serialized transaction and transaction signature.
 */
export type HaneulSignTransactionFeature = {
  /** Namespace for the feature. */
  "haneul:signTransaction": {
    /** Version of the feature API. */
    version: HaneulSignTransactionVersion;
    signTransaction: HaneulSignTransactionMethod;
  };
};

export type HaneulSignTransactionMethod = (
  input: HaneulSignTransactionInput
) => Promise<HaneulSignTransactionOutput>;

/** Input for signing transactions. */
export interface HaneulSignTransactionInput {
  transaction: SignableTransaction;
  options?: HaneulSignTransactionOptions;
}

/** Output of signing transactions. */
export interface HaneulSignTransactionOutput extends SignedTransaction {}

/** Options for signing transactions. */
export interface HaneulSignTransactionOptions {}
