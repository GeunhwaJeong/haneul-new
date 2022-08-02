// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// invalid, struct type has type param

//# init --addresses test=0x0

//# publish
module test::m {

    struct M<phantom T> has drop { }

    fun init<T>(_: M<T>, _ctx: &mut haneul::tx_context::TxContext) {
    }
}
