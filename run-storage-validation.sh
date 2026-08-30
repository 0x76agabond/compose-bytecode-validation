#!/usr/bin/env bash
set -euo pipefail

project_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$project_dir"

forge build \
  --force \
  --out tests/fixtures/foundry-out \
  tests/fixtures/evmole

cargo run --features fixture --example storage_validation_fixtures
