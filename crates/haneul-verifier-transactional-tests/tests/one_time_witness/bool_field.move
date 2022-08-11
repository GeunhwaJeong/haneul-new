// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// correct, bool field specified at source level

//# init --addresses test=0x0

//# publish
module test::m {

    struct M has drop { some_field: bool }

    fun init(_: M, _ctx: &mut haneul::tx_context::TxContext) {
    }
}
