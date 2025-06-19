// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

pub const TEST_DIR: &str = "tests";
use haneul_transactional_test_runner::run_ptb_v2_test;

// NOTE! These tests are enabled per-directory via `haneul_transactional_test_runner::run_ptb_v2_test`
datatest_stable::harness!(run_ptb_v2_test, TEST_DIR, r".*\.(mvir|move)$");
