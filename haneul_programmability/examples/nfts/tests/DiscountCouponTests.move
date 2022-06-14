// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module NFTs::DiscountCouponTests {
    use NFTs::DiscountCoupon::{Self, DiscountCoupon};
    use Haneul::Coin::{Self, Coin};
    use Haneul::HANEUL::HANEUL;
    use Haneul::TestScenario::Self;
    use Haneul::TxContext::TxContext;

    const ISSUER_ADDRESS: address = @0xA001;
    const USER1_ADDRESS: address = @0xB001;

    // Error codes.
    // const MINT_FAILED: u64 = 0;
    // const TRANSFER_FAILED: u64 = 1;

    // Initializes the "state of the world" that mimics what should
    // be available in Haneul genesis state (e.g., mints and distributes
    // coins to users).
    fun init(ctx: &mut TxContext) {
        let coin = Coin::mint_for_testing(100, ctx);
        Coin::transfer<HANEUL>(coin, ISSUER_ADDRESS);
    }

    #[test]
    fun test_mint_then_transfer() {
        let scenario = &mut TestScenario::begin(&ISSUER_ADDRESS);
        {
            init(TestScenario::ctx(scenario));
        };

        // Mint and transfer NFT + top up recipient's address.
        TestScenario::next_tx(scenario, &ISSUER_ADDRESS);
        {
            let coin = TestScenario::take_owned<Coin<HANEUL>>(scenario);
            DiscountCoupon::mint_and_topup(coin, 10, 1648820870, USER1_ADDRESS, TestScenario::ctx(scenario));
        };

        TestScenario::next_tx(scenario, &USER1_ADDRESS);
        {
            assert!(TestScenario::can_take_owned<DiscountCoupon>(scenario), 0);
            let nft_coupon = TestScenario::take_owned<DiscountCoupon>(scenario); // if can remove, object exists
            assert!(DiscountCoupon::issuer(&nft_coupon) == ISSUER_ADDRESS, 0);
            TestScenario::return_owned(scenario, nft_coupon);
        }
    }
}
