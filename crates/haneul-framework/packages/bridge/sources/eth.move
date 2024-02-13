// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module bridge::eth {
    use std::option;

    use haneul::coin;
    use haneul::coin::TreasuryCap;
    use haneul::transfer;
    use haneul::tx_context::TxContext;

    friend bridge::treasury;

    struct ETH has drop {}

    public(friend) fun create(ctx: &mut TxContext): TreasuryCap<ETH> {
        let (treasury_cap, metadata) = coin::create_currency(
            ETH {},
            // ETC DP limited to 8 on Haneul
            8,
            b"ETH",
            b"Ethereum",
            b"Bridged Ethereum token",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        treasury_cap
    }
}
