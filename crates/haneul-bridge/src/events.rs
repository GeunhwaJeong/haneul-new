// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! This file contains the definition of the HaneulBridgeEvent enum, of
//! which each variant is an emitted Event struct defind in the Move
//! Bridge module. We rely on structures in this file to decode
//! the bcs content of the emitted events.

#![allow(non_upper_case_globals)]

use crate::crypto::BridgeAuthorityPublicKey;
use crate::error::BridgeError;
use crate::error::BridgeResult;
use crate::types::BridgeAction;
use crate::types::HaneulToEthBridgeAction;
use ethers::types::Address as EthAddress;
use fastcrypto::encoding::Encoding;
use fastcrypto::encoding::Hex;
use move_core_types::language_storage::StructTag;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use haneul_json_rpc_types::HaneulEvent;
use haneul_types::base_types::HaneulAddress;
use haneul_types::bridge::BridgeChainId;
use haneul_types::bridge::MoveTypeBridgeMessageKey;
use haneul_types::bridge::MoveTypeCommitteeMember;
use haneul_types::bridge::MoveTypeCommitteeMemberRegistration;
use haneul_types::collection_types::VecMap;
use haneul_types::crypto::ToFromBytes;
use haneul_types::digests::TransactionDigest;
use haneul_types::BRIDGE_PACKAGE_ID;

// `TokendDepositedEvent` emitted in bridge.move
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct MoveTokenDepositedEvent {
    pub seq_num: u64,
    pub source_chain: u8,
    pub sender_address: Vec<u8>,
    pub target_chain: u8,
    pub target_address: Vec<u8>,
    pub token_type: u8,
    pub amount_haneul_adjusted: u64,
}

// `TokenTransferApproved` emitted in bridge.move
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct MoveTokenTransferApproved {
    pub message_key: MoveTypeBridgeMessageKey,
}

// `TokenTransferClaimed` emitted in bridge.move
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct MoveTokenTransferClaimed {
    pub message_key: MoveTypeBridgeMessageKey,
}

// `TokenTransferAlreadyApproved` emitted in bridge.move
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct MoveTokenTransferAlreadyApproved {
    pub message_key: MoveTypeBridgeMessageKey,
}

// `TokenTransferAlreadyClaimed` emitted in bridge.move
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct MoveTokenTransferAlreadyClaimed {
    pub message_key: MoveTypeBridgeMessageKey,
}

// `CommitteeUpdateEvent` emitted in committee.move
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MoveCommitteeUpdateEvent {
    pub members: VecMap<Vec<u8>, MoveTypeCommitteeMember>,
    pub stake_participation_percentage: u64,
}

// `BlocklistValidatorEvent` emitted in committee.move
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MoveBlocklistValidatorEvent {
    pub blocklisted: bool,
    pub public_keys: Vec<Vec<u8>>,
}

// Sanitized version of MoveTokenDepositedEvent
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct EmittedHaneulToEthTokenBridgeV1 {
    pub nonce: u64,
    pub haneul_chain_id: BridgeChainId,
    pub eth_chain_id: BridgeChainId,
    pub haneul_address: HaneulAddress,
    pub eth_address: EthAddress,
    pub token_id: u8,
    // The amount of tokens deposited with decimal points on Haneul side
    pub amount_haneul_adjusted: u64,
}

// Sanitized version of MoveTokenTransferApproved
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct TokenTransferApproved {
    pub nonce: u64,
    pub source_chain: BridgeChainId,
}

impl TryFrom<MoveTokenTransferApproved> for TokenTransferApproved {
    type Error = BridgeError;

    fn try_from(event: MoveTokenTransferApproved) -> BridgeResult<Self> {
        let source_chain = BridgeChainId::try_from(event.message_key.source_chain).map_err(|_e| {
            BridgeError::Generic(format!(
                "Failed to convert MoveTokenTransferApproved to TokenTransferApproved. Failed to convert source chain {} to BridgeChainId",
                event.message_key.source_chain,
            ))
        })?;
        Ok(Self {
            nonce: event.message_key.bridge_seq_num,
            source_chain,
        })
    }
}

// Sanitized version of MoveTokenTransferClaimed
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct TokenTransferClaimed {
    pub nonce: u64,
    pub source_chain: BridgeChainId,
}

impl TryFrom<MoveTokenTransferClaimed> for TokenTransferClaimed {
    type Error = BridgeError;

    fn try_from(event: MoveTokenTransferClaimed) -> BridgeResult<Self> {
        let source_chain = BridgeChainId::try_from(event.message_key.source_chain).map_err(|_e| {
            BridgeError::Generic(format!(
                "Failed to convert MoveTokenTransferClaimed to TokenTransferClaimed. Failed to convert source chain {} to BridgeChainId",
                event.message_key.source_chain,
            ))
        })?;
        Ok(Self {
            nonce: event.message_key.bridge_seq_num,
            source_chain,
        })
    }
}

// Sanitized version of MoveTokenTransferAlreadyApproved
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct TokenTransferAlreadyApproved {
    pub nonce: u64,
    pub source_chain: BridgeChainId,
}

impl TryFrom<MoveTokenTransferAlreadyApproved> for TokenTransferAlreadyApproved {
    type Error = BridgeError;

    fn try_from(event: MoveTokenTransferAlreadyApproved) -> BridgeResult<Self> {
        let source_chain = BridgeChainId::try_from(event.message_key.source_chain).map_err(|_e| {
            BridgeError::Generic(format!(
                "Failed to convert MoveTokenTransferAlreadyApproved to TokenTransferAlreadyApproved. Failed to convert source chain {} to BridgeChainId",
                event.message_key.source_chain,
            ))
        })?;
        Ok(Self {
            nonce: event.message_key.bridge_seq_num,
            source_chain,
        })
    }
}

// Sanitized version of MoveTokenTransferAlreadyClaimed
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct TokenTransferAlreadyClaimed {
    pub nonce: u64,
    pub source_chain: BridgeChainId,
}

impl TryFrom<MoveTokenTransferAlreadyClaimed> for TokenTransferAlreadyClaimed {
    type Error = BridgeError;

    fn try_from(event: MoveTokenTransferAlreadyClaimed) -> BridgeResult<Self> {
        let source_chain = BridgeChainId::try_from(event.message_key.source_chain).map_err(|_e| {
            BridgeError::Generic(format!(
                "Failed to convert MoveTokenTransferAlreadyClaimed to TokenTransferAlreadyClaimed. Failed to convert source chain {} to BridgeChainId",
                event.message_key.source_chain,
            ))
        })?;
        Ok(Self {
            nonce: event.message_key.bridge_seq_num,
            source_chain,
        })
    }
}

// Sanitized version of MoveCommitteeUpdateEvent
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct CommitteeUpdate {
    pub members: Vec<MoveTypeCommitteeMember>,
    pub stake_participation_percentage: u64,
}

impl TryFrom<MoveCommitteeUpdateEvent> for CommitteeUpdate {
    type Error = BridgeError;

    fn try_from(event: MoveCommitteeUpdateEvent) -> BridgeResult<Self> {
        let members = event
            .members
            .contents
            .into_iter()
            .map(|v| v.value)
            .collect();
        Ok(Self {
            members,
            stake_participation_percentage: event.stake_participation_percentage,
        })
    }
}

// Sanitized version of MoveBlocklistValidatorEvent
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct BlocklistValidatorEvent {
    pub blocklisted: bool,
    pub public_keys: Vec<BridgeAuthorityPublicKey>,
}

impl TryFrom<MoveBlocklistValidatorEvent> for BlocklistValidatorEvent {
    type Error = BridgeError;

    fn try_from(event: MoveBlocklistValidatorEvent) -> BridgeResult<Self> {
        let public_keys = event.public_keys.into_iter().map(|bytes|
            BridgeAuthorityPublicKey::from_bytes(&bytes).map_err(|e|
                BridgeError::Generic(format!("Failed to convert MoveBlocklistValidatorEvent to BlocklistValidatorEvent. Failed to convert public key to BridgeAuthorityPublicKey: {:?}", e))
            )
        ).collect::<BridgeResult<Vec<_>>>()?;
        Ok(Self {
            blocklisted: event.blocklisted,
            public_keys,
        })
    }
}

impl TryFrom<MoveTokenDepositedEvent> for EmittedHaneulToEthTokenBridgeV1 {
    type Error = BridgeError;

    fn try_from(event: MoveTokenDepositedEvent) -> BridgeResult<Self> {
        let token_id = event.token_type;
        let haneul_chain_id = BridgeChainId::try_from(event.source_chain).map_err(|_e| {
            BridgeError::Generic(format!(
                "Failed to convert MoveTokenDepositedEvent to EmittedHaneulToEthTokenBridgeV1. Failed to convert source chain {} to BridgeChainId",
                event.token_type,
            ))
        })?;
        let eth_chain_id = BridgeChainId::try_from(event.target_chain).map_err(|_e| {
            BridgeError::Generic(format!(
                "Failed to convert MoveTokenDepositedEvent to EmittedHaneulToEthTokenBridgeV1. Failed to convert target chain {} to BridgeChainId",
                event.token_type,
            ))
        })?;

        match haneul_chain_id {
            BridgeChainId::HaneulMainnet | BridgeChainId::HaneulTestnet | BridgeChainId::HaneulCustom => {}
            _ => {
                return Err(BridgeError::Generic(format!(
                    "Failed to convert MoveTokenDepositedEvent to EmittedHaneulToEthTokenBridgeV1. Invalid source chain {}",
                    event.source_chain
                )));
            }
        }
        match eth_chain_id {
            BridgeChainId::EthMainnet | BridgeChainId::EthSepolia | BridgeChainId::EthCustom => {}
            _ => {
                return Err(BridgeError::Generic(format!(
                    "Failed to convert MoveTokenDepositedEvent to EmittedHaneulToEthTokenBridgeV1. Invalid target chain {}",
                    event.target_chain
                )));
            }
        }

        let haneul_address = HaneulAddress::from_bytes(event.sender_address)
            .map_err(|e| BridgeError::Generic(format!("Failed to convert MoveTokenDepositedEvent to EmittedHaneulToEthTokenBridgeV1. Failed to convert sender_address to HaneulAddress: {:?}", e)))?;
        let eth_address = EthAddress::from_str(&Hex::encode(&event.target_address))?;

        Ok(Self {
            nonce: event.seq_num,
            haneul_chain_id,
            eth_chain_id,
            haneul_address,
            eth_address,
            token_id,
            amount_haneul_adjusted: event.amount_haneul_adjusted,
        })
    }
}

crate::declare_events!(
    HaneulToEthTokenBridgeV1(EmittedHaneulToEthTokenBridgeV1) => ("bridge::TokenDepositedEvent", MoveTokenDepositedEvent),
    TokenTransferApproved(TokenTransferApproved) => ("bridge::TokenTransferApproved", MoveTokenTransferApproved),
    TokenTransferClaimed(TokenTransferClaimed) => ("bridge::TokenTransferClaimed", MoveTokenTransferClaimed),
    TokenTransferAlreadyApproved(TokenTransferAlreadyApproved) => ("bridge::TokenTransferAlreadyApproved", MoveTokenTransferAlreadyApproved),
    TokenTransferAlreadyClaimed(TokenTransferAlreadyClaimed) => ("bridge::TokenTransferAlreadyClaimed", MoveTokenTransferAlreadyClaimed),
    // No need to define a sanitized event struct for MoveTypeCommitteeMemberRegistration
    // because the info provided by validators could be invalid
    CommitteeMemberRegistration(MoveTypeCommitteeMemberRegistration) => ("committee::CommitteeMemberRegistration", MoveTypeCommitteeMemberRegistration),
    CommitteeUpdateEvent(CommitteeUpdate) => ("committee::CommitteeUpdateEvent", MoveCommitteeUpdateEvent),
    BlocklistValidator(BlocklistValidatorEvent) => ("committee::CommitteeUpdateEvent", MoveBlocklistValidatorEvent),

    // Add new event types here. Format:
    // EnumVariantName(Struct) => ("{module}::{event_struct}", CorrespondingMoveStruct)
);

#[macro_export]
macro_rules! declare_events {
    ($($variant:ident($type:path) => ($event_tag:expr, $event_struct:path)),* $(,)?) => {

        #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
        pub enum HaneulBridgeEvent {
            $($variant($type),)*
        }

        $(pub static $variant: OnceCell<StructTag> = OnceCell::new();)*

        pub(crate) fn init_all_struct_tags() {
            $($variant.get_or_init(|| {
                StructTag::from_str(&format!("0x{}::{}", BRIDGE_PACKAGE_ID.to_hex(), $event_tag)).unwrap()
            });)*
        }

        // Try to convert a HaneulEvent into HaneulBridgeEvent
        impl HaneulBridgeEvent {
            pub fn try_from_haneul_event(event: &HaneulEvent) -> BridgeResult<Option<HaneulBridgeEvent>> {
                init_all_struct_tags(); // Ensure all tags are initialized

                // Unwrap safe: we inited above
                $(
                    if &event.type_ == $variant.get().unwrap() {
                        let event_struct: $event_struct = bcs::from_bytes(&event.bcs).map_err(|e| BridgeError::InternalError(format!("Failed to deserialize event to {}: {:?}", stringify!($event_struct), e)))?;
                        return Ok(Some(HaneulBridgeEvent::$variant(event_struct.try_into()?)));
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
            HaneulBridgeEvent::TokenTransferApproved(_event) => None,
            HaneulBridgeEvent::TokenTransferClaimed(_event) => None,
            HaneulBridgeEvent::TokenTransferAlreadyApproved(_event) => None,
            HaneulBridgeEvent::TokenTransferAlreadyClaimed(_event) => None,
            HaneulBridgeEvent::CommitteeMemberRegistration(_event) => None,
            HaneulBridgeEvent::CommitteeUpdateEvent(_event) => None,
            HaneulBridgeEvent::BlocklistValidator(_event) => None,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::e2e_tests::test_utils::BridgeTestClusterBuilder;
    use crate::types::BridgeAction;
    use crate::types::HaneulToEthBridgeAction;
    use ethers::types::Address as EthAddress;
    use haneul_json_rpc_types::HaneulEvent;
    use haneul_types::base_types::ObjectID;
    use haneul_types::base_types::HaneulAddress;
    use haneul_types::bridge::BridgeChainId;
    use haneul_types::bridge::TOKEN_ID_HANEUL;
    use haneul_types::digests::TransactionDigest;
    use haneul_types::event::EventID;
    use haneul_types::Identifier;

    /// Returns a test HaneulEvent and corresponding BridgeAction
    pub fn get_test_haneul_event_and_action(identifier: Identifier) -> (HaneulEvent, BridgeAction) {
        init_all_struct_tags(); // Ensure all tags are initialized
        let sanitized_event = EmittedHaneulToEthTokenBridgeV1 {
            nonce: 1,
            haneul_chain_id: BridgeChainId::HaneulTestnet,
            haneul_address: HaneulAddress::random_for_testing_only(),
            eth_chain_id: BridgeChainId::EthSepolia,
            eth_address: EthAddress::random(),
            token_id: TOKEN_ID_HANEUL,
            amount_haneul_adjusted: 100,
        };
        let emitted_event = MoveTokenDepositedEvent {
            seq_num: sanitized_event.nonce,
            source_chain: sanitized_event.haneul_chain_id as u8,
            sender_address: sanitized_event.haneul_address.to_vec(),
            target_chain: sanitized_event.eth_chain_id as u8,
            target_address: sanitized_event.eth_address.as_bytes().to_vec(),
            token_type: sanitized_event.token_id,
            amount_haneul_adjusted: sanitized_event.amount_haneul_adjusted,
        };

        let tx_digest = TransactionDigest::random();
        let event_idx = 10u16;
        let bridge_action = BridgeAction::HaneulToEthBridgeAction(HaneulToEthBridgeAction {
            haneul_tx_digest: tx_digest,
            haneul_tx_event_index: event_idx,
            haneul_bridge_event: sanitized_event.clone(),
        });
        let event = HaneulEvent {
            type_: HaneulToEthTokenBridgeV1.get().unwrap().clone(),
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

    #[tokio::test]
    async fn test_bridge_events_conversion() {
        telemetry_subscribers::init_for_testing();
        init_all_struct_tags();
        let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
            .with_eth_env(true)
            .with_bridge_cluster(false)
            .build()
            .await;

        let events = bridge_test_cluster
            .new_bridge_events(
                HashSet::from_iter([
                    CommitteeMemberRegistration.get().unwrap().clone(),
                    CommitteeUpdateEvent.get().unwrap().clone(),
                ]),
                false,
            )
            .await;
        for event in events.iter() {
            match HaneulBridgeEvent::try_from_haneul_event(event).unwrap().unwrap() {
                HaneulBridgeEvent::CommitteeMemberRegistration(_event) => (),
                HaneulBridgeEvent::CommitteeUpdateEvent(_event) => (),
                _ => panic!(
                    "Expected CommitteeMemberRegistration or CommitteeUpdateEvent, got {:?}",
                    event
                ),
            }
        }

        // TODO: trigger other events and make sure they are converted correctly
    }
}
