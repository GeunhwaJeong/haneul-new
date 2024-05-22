// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module haneul::coin_balance_tests {
    use haneul::test_scenario;
    use haneul::pay;
    use haneul::coin;
    use haneul::balance;
    use haneul::haneul::HANEUL;

    #[test]
    fun type_morphing() {
        let mut scenario = test_scenario::begin(@0x1);

        let balance = balance::zero<HANEUL>();
        let coin = balance.into_coin(scenario.ctx());
        let balance = coin.into_balance();

        balance.destroy_zero();

        let mut coin = coin::mint_for_testing<HANEUL>(100, scenario.ctx());
        let balance_mut = coin::balance_mut(&mut coin);
        let sub_balance = balance_mut.split(50);

        assert!(sub_balance.value() == 50);
        assert!(coin.value() == 50);

        let mut balance = coin.into_balance();
        balance.join(sub_balance);

        assert!(balance.value() == 100);

        let coin = balance.into_coin(scenario.ctx());
        pay::keep(coin, scenario.ctx());
        scenario.end();
    }
}
