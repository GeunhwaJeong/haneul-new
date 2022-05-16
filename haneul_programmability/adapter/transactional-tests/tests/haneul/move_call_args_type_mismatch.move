// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//# init --addresses Test=0x0

//# publish

module Test::M {
    use Haneul::TxContext::TxContext;
    public(script) fun create(_value: u64, _recipient: address, _ctx: &mut TxContext) {}

}

// wrong number of args
//# run Test::M::create --args 10

// wrong arg types
//# run Test::M::create --args 10 10
