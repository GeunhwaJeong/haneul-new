# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# tests that haneul move new followed by haneul move build succeeds

haneul move new example
cd example && haneul move build
