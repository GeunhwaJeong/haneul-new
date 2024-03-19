#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0
#
# Automatically update all snapshots. This is needed when the framework is changed or when protocol config is changed.

set -x
set -e

SCRIPT_PATH=$(realpath "$0")
SCRIPT_DIR=$(dirname "$SCRIPT_PATH")
ROOT="$SCRIPT_DIR/.."

cd "$ROOT/crates/haneul-protocol-config" && cargo insta test --review
cd "$ROOT/crates/haneul-swarm-config" && cargo insta test --review
cd "$ROOT/crates/haneul-open-rpc" && cargo run --example generate-json-rpc-spec -- record
