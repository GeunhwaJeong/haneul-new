# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# tests that building a package that implicitly depends on `Bridge` works in dev mode
haneul move build --dev -p example 2> /dev/null
