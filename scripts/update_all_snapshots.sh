#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0
#
# Automatically update all snapshots. This is needed when the framework is changed or when protocol config is changed.

set -x
set -e

if ! command -v psql > /dev/null; then
  echo "Error: PostgreSQL is not installed. Please follow crates/haneul-graphql-e2e-tests/README.md" >&2
  exit 1
fi

SCRIPT_PATH=$(realpath "$0")
SCRIPT_DIR=$(dirname "$SCRIPT_PATH")
ROOT="$SCRIPT_DIR/.."

cd "$ROOT/crates/haneul-protocol-config" && cargo insta test --review
cd "$ROOT/crates/haneul-swarm-config" && cargo insta test --review
cd "$ROOT/crates/haneul-open-rpc" && cargo run --example generate-json-rpc-spec -- record
cd "$ROOT/crates/haneul-graphql-e2e-tests" && env UB=1 cargo nextest run --features pg_integration
