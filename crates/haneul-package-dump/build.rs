// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

fn main() {
    cynic_codegen::register_schema("haneul")
        .from_sdl_file("../haneul-indexer-alt-graphql/schema.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
