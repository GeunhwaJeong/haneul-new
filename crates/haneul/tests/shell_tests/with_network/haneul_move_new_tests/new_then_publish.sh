# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# tests that haneul move new followed by haneul move publish succeeds on network defined by current branch

haneul move new example
echo "module example::example;" >> example/sources/example.move

echo "=== publish package (localnet) ===" | tee /dev/stderr
haneul client --client.config $CONFIG publish "example" \
  --json 2> /dev/null > output
cat output | jq '.effects.status'
UPGRADE_CAP=$(cat output | jq -r '.objectChanges[] | select(.objectType == "0x2::package::UpgradeCap") | .objectId')

echo "=== upgrade package (localnet) ===" | tee /dev/stderr
haneul client --client.config $CONFIG upgrade --upgrade-capability $UPGRADE_CAP example \
  --json 2> /dev/null | jq '.effects.status'

echo "=== set up networks ===" | tee /dev/stderr
haneul client --client.config $CONFIG new-env --alias devnet --rpc https://fullnode.devnet.haneul.io:443
haneul client --client.config $CONFIG new-env --alias testnet --rpc https://fullnode.testnet.haneul.io:443
haneul client --client.config $CONFIG new-env --alias mainnet --rpc https://fullnode.mainnet.haneul.io:443

for i in devnet testnet mainnet; do
  echo "=== publish package ($i) ===" | tee /dev/stderr
  haneul client --client.config $CONFIG switch --env "$i" \
    2> /dev/null
  haneul client --client.config $CONFIG publish "example" \
    --dry-run \
    --json 2> /dev/null | jq '.effects.status'
done
