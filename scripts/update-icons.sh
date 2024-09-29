#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
PROJECT_DIR=$(dirname "$SCRIPT_DIR")
TMP=$(mktemp -d)
GENERATED_CODE_FILE="$TMP/generated-code.txt"
RUST_FILE="$PROJECT_DIR/crates/file_web_devicon_lib/src/icons.rs"
DELIM_PATTERN='\/\/ BEGIN GENERATED CODE'

cd "$PROJECT_DIR"

# clone devicons
git submodule update --init

nvim -l "$SCRIPT_DIR/extract-icons.lua" >"$GENERATED_CODE_FILE"

echo "$DELIM_PATTERN"
echo "$GENERATED_CODE_FILE"
echo "$RUST_FILE"

sed -i.bak -n -e "1,/$DELIM_PATTERN/ p" -e "/$DELIM_PATTERN/ r $GENERATED_CODE_FILE" "$RUST_FILE"
rm "$RUST_FILE".bak

cargo fmt --check --all

# vim: ts=4 et:
