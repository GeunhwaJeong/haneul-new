// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![allow(unused_imports)]
#![allow(unused_variables)]
use std::path::Path;
use haneul_transactional_test_runner::{
    run_test_impl,
    test_adapter::{HaneulTestAdapter, PRE_COMPILED},
};
pub const TEST_DIR: &str = "tests";

datatest_stable::harness!(run_test, TEST_DIR, r".*\.(mvir|move)$");

#[cfg_attr(not(msim), tokio::main)]
#[cfg_attr(msim, msim::main)]
async fn run_test(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(feature = "pg_integration") {
        run_test_impl::<HaneulTestAdapter>(path, Some(&*PRE_COMPILED)).await?;
    }
    Ok(())
}
