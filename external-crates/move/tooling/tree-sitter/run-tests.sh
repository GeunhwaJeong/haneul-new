#!/bin/bash

HANEUL_FRAMEWORK_DIR="../../../../crates/haneul-framework/packages/haneul-framework/**/*.move"
STDLIB_DIR="../../../../haneul-framework/packages/move-stdlib/**/*.move"

tree-sitter generate --no-bindings
tree-sitter parse -q -t tests/*.move
tree-sitter parse -q -t tree-sitter $HANEUL_FRAMEWORK_DIR
