// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type {
  ExecuteTransactionRequestType,
  HaneulTransactionResponse,
  HaneulTransactionResponseOptions,
} from "@haneullabs/haneul.js";
import type { HaneulSignTransactionInput } from "./haneulSignTransaction";

/** The latest API version of the signAndExecuteTransaction API. */
export type HaneulSignAndExecuteTransactionVersion = "2.0.0";

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
  extends HaneulSignTransactionInput {
  /**
   * `WaitForEffectsCert` or `WaitForLocalExecution`, see details in `ExecuteTransactionRequestType`.
   * Defaults to `WaitForLocalExecution` if options.showEffects or options.showEvents is true
   */
  requestType?: ExecuteTransactionRequestType;
  /** specify which fields to return (e.g., transaction, effects, events, etc). By default, only the transaction digest will be returned. */
  options?: HaneulTransactionResponseOptions;
}

/** Output of signing and sending transactions. */
export interface HaneulSignAndExecuteTransactionOutput
  extends HaneulTransactionResponse {}
