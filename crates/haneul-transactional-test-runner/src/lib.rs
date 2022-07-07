// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! This module contains the transactional test runner instantiation for the Haneul adapter

pub mod args;
pub mod test_adapter;

use move_transactional_test_runner::framework::run_test_impl;
use std::path::Path;
use test_adapter::{HaneulTestAdapter, PRE_COMPILED};

pub fn run_test(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    run_test_impl::<HaneulTestAdapter>(path, Some(&*PRE_COMPILED))
}
