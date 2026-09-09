#!/usr/bin/env bash
set -euo pipefail

project_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/tests/fixtures/evmole/9-delegatecall/project" && pwd)"
rpc_url="http://127.0.0.1:8546"
private_key="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
anvil_log="$(mktemp)"
deploy_log="$(mktemp)"

cleanup() {
  if [[ -n "${anvil_pid:-}" ]]; then
    kill "$anvil_pid" 2>/dev/null || true
    wait "$anvil_pid" 2>/dev/null || true
  fi
  rm -f "$anvil_log" "$deploy_log"
}
trap cleanup EXIT

cd "$project_dir"
if [[ ! -d lib/Compose/src || ! -d lib/forge-std/src ]]; then
  git init >/dev/null
  forge install \
    foundry-rs/forge-std@bf647bd6046f2f7da30d0c2bf435e5c76a780c1b \
    Perfect-Abstractions/Compose@82a71731ffed49c3c1f08fc03851ebfaab47a1a7
fi
forge build --force

anvil --port 8546 --silent >"$anvil_log" 2>&1 &
anvil_pid=$!
until cast block-number --rpc-url "$rpc_url" >/dev/null 2>&1; do
  if ! kill -0 "$anvil_pid" 2>/dev/null; then
    cat "$anvil_log" >&2
    exit 1
  fi
  sleep 0.1
done

forge script script/Deploy.s.sol:DeployScript \
  --rpc-url "$rpc_url" \
  --private-key "$private_key" \
  --broadcast | tee "$deploy_log"

diamond="$(awk '/Diamond:/ { address=$NF } END { print address }' "$deploy_log")"
delegate_facet="$(awk '/DelegateCallFacet:/ { address=$NF } END { print address }' "$deploy_log")"
stored_implementation="$(awk '/StoredDiamondDelegateImplementation:/ { address=$NF } END { print address }' "$deploy_log")"
if [[ ! "$diamond" =~ ^0x[0-9a-fA-F]{40}$ ]]; then
  echo "Could not recover Diamond address from deployment output" >&2
  exit 1
fi
if [[ ! "$delegate_facet" =~ ^0x[0-9a-fA-F]{40}$ ]]; then
  echo "Could not recover DelegateCallFacet address from deployment output" >&2
  exit 1
fi
if [[ ! "$stored_implementation" =~ ^0x[0-9a-fA-F]{40}$ ]]; then
  echo "Could not recover StoredDiamondDelegateImplementation address from deployment output" >&2
  exit 1
fi

cast send "$diamond" \
  'configureStoredTarget(bytes4)' \
  0xdecafbad \
  --rpc-url "$rpc_url" \
  --private-key "$private_key" >/dev/null

cast send "$diamond" \
  'delegateSetFacet(bytes4,address)' \
  0xaabbccdd \
  0x000000000000000000000000000000000000beef \
  --rpc-url "$rpc_url" \
  --private-key "$private_key" >/dev/null

cast send "$diamond" \
  'delegateOverwriteFacet(bytes4,uint256)' \
  0x11223344 \
  0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff \
  --rpc-url "$rpc_url" \
  --private-key "$private_key" >/dev/null

cast send "$diamond" \
  'delegateStoredSetFacet(bytes4,address)' \
  0xbbccddee \
  0x000000000000000000000000000000000000cafe \
  --rpc-url "$rpc_url" \
  --private-key "$private_key" >/dev/null

cast send "$diamond" \
  'delegateNestedSetFacet(bytes4,address)' \
  0xccddeeff \
  0x000000000000000000000000000000000000d00d \
  --rpc-url "$rpc_url" \
  --private-key "$private_key" >/dev/null

block_number="$(cast block-number --rpc-url "$rpc_url")"
block_tag="$(printf '0x%x' "$block_number")"

COMPOSE_RPC_URL="$rpc_url" \
COMPOSE_BLOCK_TAG="$block_tag" \
COMPOSE_CALLER_ADDRESS="$delegate_facet" \
  COMPOSE_STORAGE_ADDRESS="$diamond" \
COMPOSE_VSL="$(dirname "$project_dir")/canonical-vsl.json" \
COMPOSE_EXPECT_COLLISIONS=1 \
COMPOSE_EXPECT_DELEGATECALL_WARNINGS=5 \
  cargo run --features fixture,rpc --example delegatecall_validation_fixture

echo "delegatecall diamond fixture passed: $diamond"
