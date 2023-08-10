// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::*;

use super::haneul_address::HaneulAddress;

pub(crate) struct TransactionBlock;
pub(crate) struct TransactionBlockConnection;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub(crate) enum TransactionBlockKindInput {
    ConsensusCommitPrologue,
    Genesis,
    ChangeEpoch,
    Programmable,
}

#[derive(InputObject)]
pub(crate) struct TransactionBlockFilter {
    package: Option<HaneulAddress>,
    module: Option<String>,
    function: Option<String>,

    kind: Option<TransactionBlockKindInput>,
    checkpoint: Option<u64>,

    sign_address: Option<HaneulAddress>,
    sent_address: Option<HaneulAddress>,
    recv_address: Option<HaneulAddress>,
    paid_address: Option<HaneulAddress>,

    input_object: Option<HaneulAddress>,
    changed_object: Option<HaneulAddress>,
}

#[allow(unreachable_code)]
#[allow(unused_variables)]
#[Object]
impl TransactionBlock {
    async fn id(&self) -> ID {
        unimplemented!()
    }
}

#[allow(unreachable_code)]
#[allow(unused_variables)]
#[Object]
impl TransactionBlockConnection {
    async fn unimplemented(&self) -> bool {
        unimplemented!()
    }
}
