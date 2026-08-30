# Compose Bytecode Validation

Research PoC for validating a Compose facet's deployed runtime bytecode against
its Solidity-derived Virtual Storage Layout (VSL).

This is not a general-purpose Solidity decompiler and it does not attempt to
reconstruct a complete contract layout from bytecode. Compose already has a
stronger source-side model: the VSL knows declared types, packing, container
boundaries, and named storage roots. This project asks a narrower question:

> Does bytecode storage evidence clearly contradict the selected facet's VSL?

## Inputs and Verdicts

Each validation run consumes:

- a facet's runtime bytecode;
- VSL records generated from the Solidity compact AST; and
- concrete root slots derived from the VSL namespaces.

The active write validator reports four evidence collections:

| Evidence | Meaning |
| --- | --- |
| `collisions` | A persistent write contradicts a VSL slot, offset, width, or value/container shape. |
| `validatedVariables` | A recovered persistent write matches one VSL variable. |
| `uncertainScopes` | A write has a concrete storage scope but cannot be compared conclusively. |
| `diagnostics` | The engine cannot recover even a concrete persistent storage root. |

No collision is not a complete proof of safety. Unreached paths and unsupported
compiler patterns remain outside the evidence set.

## Current Validation Path

`src/storage_validation/` is the active PoC path. It consumes only persistent
`SSTORE` evidence and compares each recovered variable with the canonical
full-diamond VSL:

```text
facet runtime bytecode + canonical VSL
  -> recovered persistent writes
  -> collisions[] | validatedVariables[] | uncertainScopes[] | diagnostics[]
```

An `uncertainScope` always has a concrete recovered storage slot, selector,
program counter, and reason. A write with no recoverable root is a diagnostic,
not an unspecified global warning.

## Historical Inference Experiments

`src/compose/` runs both paths over the same raw storage trace:

- `compose`: leaves EVMole's bytecode inference independent, then compares it
  with VSL physical and semantic layout.
- `compose-vsl-bias`: tests whether a matching VSL can explain an ambiguous
  inference, such as a mapping value struct collapsed to `address`.

Every VSL bias is emitted as an assumption. A report containing an assumption
is always `uncertain`; VSL must never turn ambiguous bytecode into a safe
verdict.

Mapping key-type differences are diagnostic only. The comparison treats value
shape, container shape, byte width, packing offset, and root slot identity as
storage compatibility evidence.

## Fixture Coverage

The six fixtures in `tests/fixtures/evmole/` are write-based challenge suites.
Each directory contains a canonical Solidity contract, its checked-in canonical
VSL, and one or more incompatible contracts that use the same storage root.
Foundry builds every runtime bytecode before the runner compares each variant
against the canonical VSL.

| Fixture | Challenge | Current evidence |
| --- | --- | --- |
| `1-normal` | Packed nested-struct width shifts | Canonical writes validate; incompatible slot shifts produce collisions. |
| `2-constant-key` | Constant mapping keys and packed dynamic-array width | Mapping writes validate; element width remains scoped uncertainty. |
| `3-storage-key` | Storage-derived mapping keys and dynamic/fixed indexes | Roots and plain values validate; packed indexed writes remain scoped uncertainty. |
| `4-mapping-struct` | Reordered packed members inside a mapping value | Root is known; member path reconstruction remains scoped uncertainty. |
| `5-array-struct` | Reordered struct arrays and scalar-array replacement | Root is known; array child path reconstruction remains scoped uncertainty. |
| `6-array-mapping-struct` | Mapping-to-address versus mapping-to-array-struct | Canonical mapping validates and both incompatible container shapes collide. |

An inferred fallback `uint256` cannot prove a collision. The raw tracer marks
whether the write value type was actually recovered; fallback values are
reported only as scoped uncertainty. Dynamic-array length writes are compared
as container metadata rather than as element writes.

### Current Result Snapshot

The current assertion-backed run covers 15 contracts across the six fixture
families:

| Fixture | Variant | Collisions | Validated | Scoped uncertainty |
| --- | --- | ---: | ---: | ---: |
| `1-normal` | canonical | 0 | 7 | 4 |
| `1-normal` | incompatible packed width | 2 | 1 | 2 |
| `2-constant-key` | canonical | 0 | 3 | 1 |
| `2-constant-key` | incompatible dynamic width | 0 | 3 | 1 |
| `3-storage-key` | canonical | 0 | 4 | 4 |
| `3-storage-key` | incompatible dynamic width | 0 | 0 | 2 |
| `3-storage-key` | incompatible fixed width | 0 | 0 | 2 |
| `4-mapping-struct` | canonical | 0 | 0 | 3 |
| `4-mapping-struct` | incompatible reordered members | 0 | 0 | 3 |
| `5-array-struct` | canonical | 0 | 0 | 3 |
| `5-array-struct` | incompatible reordered members | 0 | 0 | 3 |
| `5-array-struct` | incompatible scalar array | 0 | 0 | 1 |
| `6-array-mapping-struct` | canonical | 0 | 1 | 0 |
| `6-array-mapping-struct` | incompatible array-only value | 1 | 0 | 1 |
| `6-array-mapping-struct` | incompatible array-and-fields value | 1 | 1 | 1 |

All variants currently complete without an unresolved-root diagnostic. The
engine reliably tracks root slots, static slot shifts, selectors, program
counters, constant mapping keys, and storage-derived mapping keys. It also
proves container-shape contradictions in case 6.

The next implementation target is symbolic child-path reconstruction. Packed
dynamic/fixed array element widths and struct members inside mappings or arrays
currently retain a concrete root but not enough member/index/stride information
to challenge the corresponding VSL child. Cases 2 through 5 therefore remain
scoped uncertainty where the current engine cannot prove a contradiction.

## Run the PoC

Build all write fixtures with Foundry and run the assertion-backed comparison:

```sh
./run-storage-validation.sh
```

Set `COMPOSE_FIXTURE_CASE=4-mapping-struct` to run one case. Set
`COMPOSE_TRACE_STORAGE=1` to print raw storage evidence before comparison.

Run the test suite:

```sh
cargo test --features fixture
```

## Generate VSL Inputs

`tools/generate-vsl.mts` compiles one Solidity source with Foundry's `--ast`
output and writes canonical VSL JSON. The tool-local VSL builder is
copied from Compose CLI and preserves a readable `virtualPath`; its `id` is
canonicalized with `cast keccak` so it can match physical EVM storage roots.

It requires Foundry (`forge`, `cast`) and `tsx` from a Compose CLI checkout.
From this repository in the current research workspace:

```sh
node ../../../Compose/cli/node_modules/tsx/dist/cli.mjs tools/generate-vsl.mts \
  tests/fixtures/evmole/1-normal/Canonical.sol Case1Canonical \
  --out tests/fixtures/evmole/1-normal/canonical-vsl.json
```

Set `FOUNDRY_FORGE` or `FOUNDRY_CAST` when the executables are not on `PATH`.

## Architecture

[Architecture.md](./Architecture.md) documents the inherited symbolic-execution
engine and the Compose validation boundary. The main extension points are:

- `src/storage/mod.rs`: raw storage evidence before EVMole collapses records;
- `src/storage_validation/`: active VSL-driven persistent-write validator;
- `src/compose/`: historical unbiased and VSL-bias comparison experiments;
- `tools/`: VSL generation from Solidity AST;
- `tests/fixtures/evmole/`: canonical VSL and incompatible bytecode challenge
  sources for each research case.

## Provenance

This repository began as a fork of
[EVMole](https://github.com/cdump/evmole) v0.9.3. The EVM interpreter,
selector/argument recovery, and storage tracer remain the analysis substrate;
Compose-specific validation is added as a separate host layer. The original
MIT license is retained in [LICENSE](./LICENSE).
