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

## What Differs From Original EVMole

Original EVMole is a general bytecode-to-Solidity recovery tool. Its storage
output is designed to summarize inferred contract state, so it may intentionally
collapse `KECCAK256 + constant` accesses back to a mapping or array root.

This fork keeps EVMole's interpreter, CFG, ABI argument recovery, and symbolic
storage tracer as the analysis substrate, then adds a Compose-specific layer:

- it accepts the canonical full-diamond VSL as an external input rather than
  inferring the entire source layout from bytecode;
- it retains `SSTORE` evidence needed for contradiction checks, including
  persistent root slot, constant mapping-value slot delta, packed byte offset,
  field width, selector, and program counter;
- it compares only concrete writes against declared VSL variables and returns
  a proven collision, validation, scoped uncertainty, or unresolved-root
  diagnostic; and
- it preserves generic-decompiler uncertainty instead of letting VSL turn an
  ambiguous bytecode trace into a safe verdict.

The scope is deliberately narrower than a complete decompiler: the validator
tries to prove a bytecode/VSL contradiction, not reconstruct every storage
variable that a facet could access.

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
| `2-constant-key` | Constant mapping keys and packed dynamic-array width | VSL anchors constant keys; canonical array writes validate and width mismatch collides. |
| `3-storage-key` | Storage-derived mapping keys and dynamic/fixed indexes | VSL anchors the loaded `uint64` key; dynamic and fixed packed element mismatches collide. |
| `4-mapping-struct` | Reordered packed members inside a mapping value | Mapping-value child slots and packed fields validate; reordered members collide. |
| `5-array-struct` | Reordered struct arrays and scalar-array replacement | Root is known; array child path reconstruction remains scoped uncertainty. |
| `6-array-mapping-struct` | Mapping-to-address versus mapping-to-array-struct | Mapping value roots are recovered; missing child paths remain scoped uncertainty. |

An inferred fallback `uint256` cannot prove a collision. The raw tracer marks
whether the write value type was actually recovered; fallback values are
reported only as scoped uncertainty. Dynamic-array length writes are compared
as container metadata rather than as element writes.

### Current Result Snapshot

The current assertion-backed run covers 15 contracts across the six fixture
families:

| Fixture | Variant | Collisions | Validated | Scoped uncertainty |
| --- | --- | ---: | ---: | ---: |
| `1-normal` | canonical | 0 | 9 | 2 |
| `1-normal` | incompatible packed width | 2 | 3 | 0 |
| `2-constant-key` | canonical | 0 | 2 | 2 |
| `2-constant-key` | incompatible dynamic width | 1 | 1 | 2 |
| `3-storage-key` | canonical | 0 | 7 | 1 |
| `3-storage-key` | incompatible dynamic width | 2 | 0 | 0 |
| `3-storage-key` | incompatible fixed width | 2 | 0 | 0 |
| `4-mapping-struct` | canonical | 0 | 5 | 0 |
| `4-mapping-struct` | incompatible reordered members | 2 | 3 | 0 |
| `5-array-struct` | canonical | 0 | 0 | 3 |
| `5-array-struct` | incompatible reordered members | 0 | 0 | 3 |
| `5-array-struct` | incompatible scalar array | 0 | 0 | 1 |
| `6-array-mapping-struct` | canonical | 0 | 0 | 1 |
| `6-array-mapping-struct` | incompatible array-only value | 0 | 0 | 2 |
| `6-array-mapping-struct` | incompatible array-and-fields value | 0 | 0 | 4 |

All variants currently complete without an unresolved-root diagnostic. The
engine reliably tracks root slots, static slot shifts, selectors, program
counters, constant mapping keys, and storage-derived mapping keys.

VSL-derived trace hints now recover mapping key types for constant and
storage-loaded keys. Packed read-modify-write values retain their ABI type
through dynamic index shifting, including Solidity's boolean normalization.

Mapping-value structs now retain constant child-slot deltas from `KECCAK256 +
constant` and packed write masks. Case 4 therefore validates the canonical
mapping value and proves the reordered member contradiction. The next target is
array child-path reconstruction: case 5 still has a concrete root but not the
member index/stride information required to challenge its VSL child. Other
mapping values without a virtual child, including case 6, remain explicitly
scoped uncertainty rather than false collisions.

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
