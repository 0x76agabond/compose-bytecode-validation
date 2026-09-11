#!/usr/bin/env bash
set -euo pipefail

project_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$project_dir"

fixture_root="tests/fixtures/evmole"

# Fixture 9 is a complete Foundry project with its own dependencies and Anvil
# runner. The direct storage suite compiles only its standalone Solidity cases.
forge build \
  --force \
  --out tests/fixtures/foundry-out \
  "$fixture_root/1-normal" \
  "$fixture_root/2-constant-key" \
  "$fixture_root/3-storage-key" \
  "$fixture_root/4-mapping-struct" \
  "$fixture_root/5-array-struct" \
  "$fixture_root/5.1-mapping-struct" \
  "$fixture_root/6-array-mapping-struct" \
  "$fixture_root/7-array-mapping-struct-push" \
  "$fixture_root/8-bytes-string" \
  "$fixture_root/8.1-string-array-struct-bytes" \
  "$fixture_root/10-total-assembly" \
  "$fixture_root/final-full-storage"

cargo run --features fixture --example storage_validation_fixtures
