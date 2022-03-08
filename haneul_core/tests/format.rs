// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test]
fn test_format() {
    // If this test breaks and you intended a format change, you need to run to get the fresh format:
    // # cargo -q run --example generate-format -- print > haneul_core/tests/staged/haneul.yaml

    let status = std::process::Command::new("cargo")
        .current_dir("..")
        .args(&["run", "--example", "generate-format", "--"])
        .arg("test")
        .status()
        .expect("failed to execute process");
    assert!(status.success());
}
