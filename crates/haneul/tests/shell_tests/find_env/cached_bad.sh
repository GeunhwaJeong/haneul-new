# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# If the active environment name exists but doesn't match on chain ID, fail
haneul move --client.config configs/name_match_id_mismatch.yaml build
