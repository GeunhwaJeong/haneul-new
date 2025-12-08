# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# check that haneul move new correctly updates existing .gitignore
mkdir example
echo "existing_ignore" > example/.gitignore
haneul move --client.config $CONFIG new example
cat example/.gitignore
