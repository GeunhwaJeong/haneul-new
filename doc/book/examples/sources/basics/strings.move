// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module examples::strings {
    use haneul::object::{Self, Info};
    use haneul::tx_context::TxContext;

    // Use this dependency to get a type wrapper for UTF-8 strings
    use haneul::utf8::{Self, String};

    /// A dummy Object that holds a String type
    struct Name has key, store {
        info: Info,

        /// Here it is - the String type
        name: String
    }

    /// Create a name Object by passing raw bytes
    public fun issue_name_nft(
        name_bytes: vector<u8>, ctx: &mut TxContext
    ): Name {
        Name {
            info: object::new(ctx),
            name: utf8::string_unsafe(name_bytes)
        }
    }
}
