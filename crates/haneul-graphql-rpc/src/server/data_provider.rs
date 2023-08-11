// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// For testing, use existing RPC as data source

use std::str::FromStr;

use crate::types::address::Address;
use crate::types::balance::Balance;
use crate::types::base64::Base64;
use crate::types::big_int::BigInt;
use crate::types::transaction_block::TransactionBlock;
use crate::types::{object::Object, haneul_address::HaneulAddress};
use async_graphql::*;
use haneul_json_rpc_types::{
    HaneulObjectDataOptions, HaneulPastObjectResponse, HaneulTransactionBlockResponseOptions,
};
use haneul_sdk::types::base_types::ObjectID;
use haneul_sdk::types::digests::TransactionDigest;
use haneul_sdk::HaneulClient;

pub(crate) async fn fetch_obj(
    cl: &HaneulClient,
    address: HaneulAddress,
    version: Option<u64>,
) -> Result<Option<Object>> {
    let oid: ObjectID = address.to_array().as_slice().try_into()?;
    let opts = HaneulObjectDataOptions::full_content();

    let g = match version {
        Some(v) => match cl
            .read_api()
            .try_get_parsed_past_object(oid, v.into(), opts)
            .await?
        {
            HaneulPastObjectResponse::VersionFound(x) => x,
            _ => return Ok(None),
        },
        None => {
            let val = cl.read_api().get_object_with_options(oid, opts).await?;
            if val.error.is_some() || val.data.is_none() {
                return Ok(None);
            }
            val.data.unwrap()
        }
    };
    Ok(Some(convert_obj(g)))
}

pub(crate) async fn fetch_balance(
    cl: &HaneulClient,
    address: &HaneulAddress,
    type_: Option<String>,
) -> Result<Balance> {
    let b = cl
        .coin_read_api()
        .get_balance(address.to_array().as_slice().try_into()?, type_)
        .await?;
    Ok(convert_bal(b))
}

fn convert_obj(s: haneul_json_rpc_types::HaneulObjectData) -> Object {
    Object {
        version: s.version.into(),
        digest: s.digest.to_string(),
        storage_rebate: s.storage_rebate,
        address: HaneulAddress::from_array(**s.object_id),
        owner: s
            .owner
            .unwrap()
            .get_owner_address()
            .map(|x| HaneulAddress::from_array(x.to_inner()))
            .ok(),
        bcs: Some(Base64::from(&bcs::to_bytes(&s.bcs).unwrap())), // TODO: is this correct?
        previous_transaction: Some(s.previous_transaction.unwrap().to_string()),
    }
}

pub(crate) async fn fetch_tx(cl: &HaneulClient, digest: &String) -> Result<Option<TransactionBlock>> {
    let tx_digest = TransactionDigest::from_str(digest)?;
    let tx = cl
        .read_api()
        .get_transaction_with_options(
            tx_digest,
            HaneulTransactionBlockResponseOptions::full_content(),
        )
        .await?;
    let sender = match tx.clone().transaction.unwrap().data {
        haneul_json_rpc_types::HaneulTransactionBlockData::V1(tx) => tx.sender,
    };
    Ok(Some(TransactionBlock {
        digest: digest.to_string(),
        sender: Some(Address {
            address: HaneulAddress::from_array(sender.to_inner()),
        }),
        bcs: Some(Base64::from(&tx.raw_transaction)),
    }))
}

fn convert_bal(b: haneul_json_rpc_types::Balance) -> Balance {
    Balance {
        coin_object_count: b.coin_object_count as u64,
        total_balance: BigInt::from_str(&format!("{}", b.total_balance)).unwrap(),
    }
}
