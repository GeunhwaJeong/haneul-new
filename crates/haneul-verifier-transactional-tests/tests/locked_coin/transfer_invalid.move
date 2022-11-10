// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// locked coins cannot be transferred in any way

//# init --addresses test=0x0

//# publish
module test::m {
    use haneul::locked_coin::LockedCoin;
    use haneul::transfer::transfer;

    struct TestCoin { }

    fun t(coin: LockedCoin<TestCoin>) {
        transfer(coin, @0x42);
    }
}

//# publish
module test::m {
    use haneul::locked_coin::LockedCoin;
    use haneul::transfer::share_object;

    struct TestCoin { }

    fun t(coin: LockedCoin<TestCoin>) {
        share_object(coin);
    }
}

//# publish
module test::m {
    use haneul::locked_coin::LockedCoin;
    use haneul::transfer::freeze_object;

    struct TestCoin { }

    fun t(coin: LockedCoin<TestCoin>) {
        freeze_object(coin);
    }
}
