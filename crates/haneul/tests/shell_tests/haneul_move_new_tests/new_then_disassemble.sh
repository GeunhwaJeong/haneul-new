# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# tests that haneul move new followed by haneul move disassemble succeeds


haneul move new example
cat > example/sources/example.move <<EOF
module example::example;

public fun foo(_ctx: &mut TxContext) {}
EOF
cd example

echo "=== Build ===" >&2
haneul move build

echo "=== Disassemble ===" >&2
haneul move disassemble build/example/bytecode_modules/example.mv
