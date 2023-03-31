// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module deepbook::usd {
    use std::option::none;

    use haneul::coin::create_currency;
    use haneul::transfer::{public_freeze_object, public_share_object};
    use haneul::tx_context::TxContext;

    const DECIMAL: u8 = 8;

    struct USD has drop {}

    fun init(witness: USD, ctx: &mut TxContext) {
        let (treasury_cap, metadata) = create_currency<USD>(witness, DECIMAL, b"USD", b"USD", b"USD", none(), ctx);
        public_freeze_object(metadata);
        public_share_object(treasury_cap);
    }
}
