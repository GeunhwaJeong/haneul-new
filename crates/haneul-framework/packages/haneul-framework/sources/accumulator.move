// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module haneul::accumulator;

const ENotSystemAddress: u64 = 0;

public struct Accumulator has key {
    id: UID,
}

#[allow(unused_function)]
fun create(ctx: &TxContext) {
    assert!(ctx.sender() == @0x0, ENotSystemAddress);

    transfer::share_object(Accumulator {
        id: object::haneul_accumulator_root_object_id(),
    })
}
