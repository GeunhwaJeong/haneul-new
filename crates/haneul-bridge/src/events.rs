// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! This file contains the definition of the HaneulBridgeEvent enum, of
//! which each variant is an emitted Event struct defind in the Move
//! Bridge module. We rely on structures in this file to decode
//! the bcs content of the emitted events.

use std::str::FromStr;

use crate::error::BridgeError;
use crate::error::BridgeResult;
use crate::types::BridgeAction;
use crate::types::BridgeChainId;
use crate::types::HaneulToEthBridgeAction;
use crate::types::TokenId;
use ethers::types::Address as EthAddress;
use move_core_types::language_storage::StructTag;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use haneul_json_rpc_types::HaneulEvent;
use haneul_types::base_types::HaneulAddress;
use haneul_types::digests::TransactionDigest;

// This is the event structure defined and emitted in Move
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct EmittedHaneulToEthTokenBridgeV1 {
    pub nonce: u64,
    pub haneul_chain_id: BridgeChainId,
    pub eth_chain_id: BridgeChainId,
    pub haneul_address: HaneulAddress,
    pub eth_address: EthAddress,
    pub token_id: TokenId,
    pub amount: u64,
}

const EMITTED_HANEUL_TO_ETH_TOKEN_BRIDGE_V1_STUCT_TAG: &str =
    "0x0b::HaneulToEthTokenBridge::HaneulToEthTokenBridge";

crate::declare_events!(
    // TODO: Placeholder, use right struct tag
    HaneulToEthTokenBridgeV1(EmittedHaneulToEthTokenBridgeV1) => EMITTED_HANEUL_TO_ETH_TOKEN_BRIDGE_V1_STUCT_TAG,
    // Add new event types here. Format: EnumVariantName(Struct) => "StructTagString",
);

#[macro_export]
macro_rules! declare_events {
    ($($variant:ident($type:path) => $tag:expr),* $(,)?) => {

        #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
        pub enum HaneulBridgeEvent {
            $($variant($type),)*
        }

        #[allow(non_upper_case_globals)]
        $(pub(crate) static $variant: OnceCell<StructTag> = OnceCell::new();)*

        pub(crate) fn init_all_struct_tags() {
            $($variant.get_or_init(|| {
                StructTag::from_str($tag).unwrap()
            });)*
        }

        // Try to convert a HaneulEvent into HaneulBridgeEvent
        impl HaneulBridgeEvent {
            pub fn try_from_haneul_event(event: &HaneulEvent) -> BridgeResult<Option<HaneulBridgeEvent>> {
                init_all_struct_tags(); // Ensure all tags are initialized

                // Unwrap safe: we inited above
                $(
                    if &event.type_ == $variant.get().unwrap() {
                        return Ok(Some(HaneulBridgeEvent::$variant(bcs::from_bytes(&event.bcs).map_err(|e| BridgeError::InternalError(format!("Failed to deserialize event to HaneulBridgeEvent: {:?}", e)))
                        ?)));
                    }
                )*
                Ok(None)
            }
        }
    };
}

impl HaneulBridgeEvent {
    pub fn try_into_bridge_action(
        self,
        haneul_tx_digest: TransactionDigest,
        haneul_tx_event_index: u16,
    ) -> Option<BridgeAction> {
        match self {
            HaneulBridgeEvent::HaneulToEthTokenBridgeV1(event) => {
                Some(BridgeAction::HaneulToEthBridgeAction(HaneulToEthBridgeAction {
                    haneul_tx_digest,
                    haneul_tx_event_index,
                    haneul_bridge_event: event.clone(),
                }))
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::{EmittedHaneulToEthTokenBridgeV1, EMITTED_HANEUL_TO_ETH_TOKEN_BRIDGE_V1_STUCT_TAG};
    use crate::types::BridgeAction;
    use crate::types::BridgeChainId;
    use crate::types::HaneulToEthBridgeAction;
    use crate::types::TokenId;
    use ethers::types::Address as EthAddress;
    use move_core_types::language_storage::StructTag;
    use std::str::FromStr;
    use haneul_json_rpc_types::HaneulEvent;
    use haneul_types::base_types::ObjectID;
    use haneul_types::base_types::HaneulAddress;
    use haneul_types::digests::TransactionDigest;
    use haneul_types::event::EventID;
    use haneul_types::Identifier;

    /// Returns a test HaneulEvent and corresponding BridgeAction
    pub fn get_test_haneul_event_and_action(identifier: Identifier) -> (HaneulEvent, BridgeAction) {
        let emitted_event = EmittedHaneulToEthTokenBridgeV1 {
            nonce: 1,
            haneul_chain_id: BridgeChainId::HaneulTestnet,
            eth_chain_id: BridgeChainId::EthSepolia,
            haneul_address: HaneulAddress::random_for_testing_only(),
            eth_address: EthAddress::random(),
            token_id: TokenId::Haneul,
            amount: 100,
        };
        let tx_digest = TransactionDigest::random();
        let event_idx = 10u16;
        let bridge_action = BridgeAction::HaneulToEthBridgeAction(HaneulToEthBridgeAction {
            haneul_tx_digest: tx_digest,
            haneul_tx_event_index: event_idx,
            haneul_bridge_event: emitted_event.clone(),
        });
        let event = HaneulEvent {
            // For this test to pass, match what is in events.rs
            type_: StructTag::from_str(EMITTED_HANEUL_TO_ETH_TOKEN_BRIDGE_V1_STUCT_TAG).unwrap(),
            bcs: bcs::to_bytes(&emitted_event).unwrap(),
            id: EventID {
                tx_digest,
                event_seq: event_idx as u64,
            },

            // The following fields do not matter as of writing,
            // but if tests start to fail, it's worth checking these fields.
            package_id: ObjectID::ZERO,
            transaction_module: identifier.clone(),
            sender: HaneulAddress::random_for_testing_only(),
            parsed_json: serde_json::json!({"test": "test"}),
            timestamp_ms: None,
        };
        (event, bridge_action)
    }
}
