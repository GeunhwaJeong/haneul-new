# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# tests that haneul move new followed by haneul move build succeeds

haneul move new example

# we mangle the generated toml file to replace the framework dependency with a local dependency
FRAMEWORK_DIR=$(echo $CARGO_MANIFEST_DIR | sed 's#/crates/haneul##g')
cat example/Move.toml \
  | sed 's#\(Haneul = .*\)git = "[^"]*", \(.*\)#\1\2#' \
  | sed 's#\(Haneul = .*\), rev = "[^"]*"\(.*\)#\1\2#' \
  | sed 's#\(Haneul = .*\)subdir = "\([^"]*\)"\(.*\)#\1local = "FRAMEWORK/\2"\3#' \
  | sed "s#\(Haneul = .*\)FRAMEWORK\(.*\)#\1$FRAMEWORK_DIR\2#" \
  > Move.toml
mv Move.toml example/Move.toml

cd example && haneul move build
