# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# tests that publishing a package with an implicit dependency on `Bridge` succeeds

echo "=== set up networks ===" | tee /dev/stderr
haneul client --client.config $CONFIG new-env --alias devnet --rpc https://fullnode.devnet.haneul.io:443
haneul client --client.config $CONFIG new-env --alias testnet --rpc https://fullnode.testnet.haneul.io:443
haneul client --client.config $CONFIG new-env --alias mainnet --rpc https://fullnode.mainnet.haneul.io:443

for i in localnet devnet testnet mainnet; do
  echo "=== publish package ($i) ===" | tee /dev/stderr
  haneul client --client.config $CONFIG switch --env "$i" \
    2> /dev/null
  haneul client --client.config $CONFIG publish "example" \
    --dry-run \
    --json 2> /dev/null | jq '.effects.status'
done
