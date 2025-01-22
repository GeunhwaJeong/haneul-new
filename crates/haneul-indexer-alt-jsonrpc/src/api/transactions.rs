// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use move_core_types::annotated_value::{MoveDatatypeLayout, MoveTypeLayout};
use haneul_indexer_alt_schema::transactions::StoredTransaction;
use haneul_json_rpc_types::{
    HaneulEvent, HaneulTransactionBlock, HaneulTransactionBlockData, HaneulTransactionBlockEvents,
    HaneulTransactionBlockResponse, HaneulTransactionBlockResponseOptions,
};
use haneul_open_rpc::Module;
use haneul_open_rpc_macros::open_rpc;
use haneul_types::{
    digests::TransactionDigest, effects::TransactionEffects, error::HaneulError, event::Event,
    signature::GenericSignature, transaction::TransactionData,
};

use crate::{
    context::Context,
    data::transactions::TransactionKey,
    error::{internal_error, invalid_params},
};

use super::rpc_module::RpcModule;

#[open_rpc(namespace = "haneul", tag = "Transactions API")]
#[rpc(server, namespace = "haneul")]
trait TransactionsApi {
    /// Fetch a transaction by its transaction digest.
    #[method(name = "getTransactionBlock")]
    async fn get_transaction_block(
        &self,
        /// The digest of the queried transaction.
        digest: TransactionDigest,
        /// Options controlling the output format.
        options: HaneulTransactionBlockResponseOptions,
    ) -> RpcResult<HaneulTransactionBlockResponse>;
}

pub(crate) struct Transactions(pub Context);

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("Transaction not found: {0}")]
    NotFound(TransactionDigest),

    #[error("Error converting to response: {0}")]
    Conversion(HaneulError),

    #[error("Error resolving type information: {0}")]
    Resolution(anyhow::Error),

    #[error("Deserialization error: {0}")]
    Deserialization(#[from] bcs::Error),
}

#[async_trait::async_trait]
impl TransactionsApiServer for Transactions {
    async fn get_transaction_block(
        &self,
        digest: TransactionDigest,
        options: HaneulTransactionBlockResponseOptions,
    ) -> RpcResult<HaneulTransactionBlockResponse> {
        let Self(ctx) = self;
        let Some(stored) = ctx
            .loader()
            .load_one(TransactionKey(digest))
            .await
            .map_err(internal_error)?
        else {
            return Err(invalid_params(Error::NotFound(digest)));
        };

        response(ctx, &stored, &options)
            .await
            .map_err(internal_error)
    }
}

impl RpcModule for Transactions {
    fn schema(&self) -> Module {
        TransactionsApiOpenRpc::module_doc()
    }

    fn into_impl(self) -> jsonrpsee::RpcModule<Self> {
        self.into_rpc()
    }
}

/// Convert the representation of a transaction from the database into the response format,
/// including the fields requested in the `options`.
pub(crate) async fn response(
    ctx: &Context,
    tx: &StoredTransaction,
    options: &HaneulTransactionBlockResponseOptions,
) -> Result<HaneulTransactionBlockResponse, Error> {
    use Error as E;

    let digest = TransactionDigest::try_from(tx.tx_digest.clone()).map_err(E::Conversion)?;
    let mut response = HaneulTransactionBlockResponse::new(digest);

    if options.show_input {
        let data: TransactionData = bcs::from_bytes(&tx.raw_transaction)?;
        let tx_signatures: Vec<GenericSignature> = bcs::from_bytes(&tx.user_signatures)?;
        response.transaction = Some(HaneulTransactionBlock {
            data: HaneulTransactionBlockData::try_from_with_package_resolver(
                data,
                ctx.package_resolver(),
            )
            .await
            .map_err(E::Resolution)?,
            tx_signatures,
        })
    }

    if options.show_raw_input {
        response.raw_transaction = tx.raw_transaction.clone();
    }

    if options.show_effects {
        let effects: TransactionEffects = bcs::from_bytes(&tx.raw_effects)?;
        response.effects = Some(effects.try_into().map_err(E::Conversion)?);
    }

    if options.show_raw_effects {
        response.raw_effects = tx.raw_effects.clone();
    }

    if options.show_events {
        let events: Vec<Event> = bcs::from_bytes(&tx.events)?;
        let mut haneul_events = Vec::with_capacity(events.len());

        for (ix, event) in events.into_iter().enumerate() {
            let layout = match ctx
                .package_resolver()
                .type_layout(event.type_.clone().into())
                .await
                .map_err(|e| E::Resolution(e.into()))?
            {
                MoveTypeLayout::Struct(s) => MoveDatatypeLayout::Struct(s),
                MoveTypeLayout::Enum(e) => MoveDatatypeLayout::Enum(e),
                _ => {
                    return Err(E::Resolution(anyhow!(
                        "Event {ix} from {digest} is not a struct or enum: {}",
                        event.type_.to_canonical_string(/* with_prefix */ true)
                    )));
                }
            };

            let haneul_event = HaneulEvent::try_from(
                event,
                digest,
                ix as u64,
                Some(tx.timestamp_ms as u64),
                layout,
            )
            .map_err(E::Conversion)?;

            haneul_events.push(haneul_event)
        }

        response.events = Some(HaneulTransactionBlockEvents { data: haneul_events });
    }

    Ok(response)
}
