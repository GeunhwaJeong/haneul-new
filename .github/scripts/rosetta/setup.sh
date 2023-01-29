#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

echo "Install binaries"
cargo install --bin haneul --path crates/haneul
cargo install --bin haneul-rosetta --path crates/haneul-rosetta

echo "run Haneul genesis"
haneul genesis

echo "generate rosetta configuration"
haneul-rosetta generate-rosetta-cli-config --online-url http://127.0.0.1:9002 --offline-url http://127.0.0.1:9003

echo "install rosetta-cli"
curl -sSfL https://raw.githubusercontent.com/coinbase/rosetta-cli/master/scripts/install.sh | sh -s