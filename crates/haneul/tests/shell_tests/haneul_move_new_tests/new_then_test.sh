# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# check that haneul move new followed by haneul move test succeeds
haneul move new example
cd example && haneul move test
