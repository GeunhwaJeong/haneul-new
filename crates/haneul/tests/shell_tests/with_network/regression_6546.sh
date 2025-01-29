# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# fixing issue https://github.com/GeunhwaJeong/haneul/issues/6546

COIN=$(haneul client --client.config $CONFIG objects   --json | jq '.[0].data.objectId')
ADDR=$(haneul client --client.config $CONFIG addresses --json | jq '.addresses[0][1]')

haneul client --client.config $CONFIG \
  call --package 0x2 --module haneul --function transfer --args $COIN $ADDR \
  --gas-budget 100000000 \
  --json | jq '.effects.status'
