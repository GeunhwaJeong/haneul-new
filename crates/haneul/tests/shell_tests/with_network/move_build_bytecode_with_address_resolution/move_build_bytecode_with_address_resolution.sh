# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

haneul client --client.config $CONFIG \
  publish simple \
  --json | jq '.effects.status'

haneul move --client.config $CONFIG \
  build --path depends_on_simple
