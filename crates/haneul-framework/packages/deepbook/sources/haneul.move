// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module deepbook::haneul {
    use std::option::none;

    use haneul::coin::create_currency;
    use haneul::transfer::{public_freeze_object, public_share_object};
    use haneul::tx_context::TxContext;

    const DECIMAL: u8 = 8;

    struct HANEUL has drop {}

    fun init(witness: HANEUL, ctx: &mut TxContext) {
        let (treasury_cap, metadata) = create_currency<HANEUL>(witness, DECIMAL, b"HANEUL", b"HANEUL", b"HANEUL", none(), ctx);
        public_freeze_object(metadata);
        public_share_object(treasury_cap);
    }
}
