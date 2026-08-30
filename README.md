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

The result is intentionally three-valued:

| Verdict | Meaning |
| --- | --- |
| `no-contradiction` | Every observed storage access is compatible and bytecode-backed. |
| `contradiction` | Bytecode shows an incompatible slot width, offset, or value/container shape. |
| `uncertain` | Evidence is missing, unsupported, or needs an explicit VSL assumption. |

`no-contradiction` is not a complete proof of safety. Unreached paths and
unsupported compiler patterns remain outside the evidence set.

## Two Experimental Paths

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

## Current Fixture Results

The six fixtures in `tests/fixtures/evmole/` are a controlled Solidity suite:

| Fixture | Raw bytecode result | VSL-biased result | Observation |
| --- | --- | --- | --- |
| `1-normal` | no contradiction | no contradiction | Normal structs, mappings, packing, and `uint8[]` are recovered. |
| `2-constant-key` | contradiction | contradiction | `uint8[]` is widened to `uint256[]`. |
| `3-storage-key` | contradiction | contradiction | Packed scalar bias helps, but packed dynamic arrays still widen. |
| `4-mapping-struct` | contradiction | uncertain | Mapping values containing structs collapse. |
| `5-array-struct` | contradiction | uncertain | Array and mapping struct values collapse. |
| `6-array-mapping-struct` | contradiction | uncertain | Distinct struct shapes can collapse to the same bytecode type. |

The suite is evidence for design decisions, not a compatibility claim about all
Solidity bytecode.

## Run the PoC

The Rust fixture runner uses checked-in bytecode and VSL JSON:

```sh
cargo run --features fixture --example compose_fixture_comparison
```

Run the test suite:

```sh
cargo test --features fixture
```

## Generate VSL Inputs

`tools/generate-vsl.mts` compiles one Solidity source with Foundry's `--ast`
output and writes `vsl.json` beside the source. The tool-local VSL builder is
copied from Compose CLI and preserves a readable `virtualPath`; its `id` is
canonicalized with `cast keccak` so it can match physical EVM storage roots.

It requires Foundry (`forge`, `cast`) and `tsx` from a Compose CLI checkout.
From this repository in the current research workspace:

```sh
node ../../../Compose/cli/node_modules/tsx/dist/cli.mjs tools/generate-vsl.mts \
  tests/fixtures/evmole/1-normal/Normal.sol Normal
```

Set `FOUNDRY_FORGE` or `FOUNDRY_CAST` when the executables are not on `PATH`.

## Architecture

[Architecture.md](./Architecture.md) documents the inherited symbolic-execution
engine and the Compose validation boundary. The main extension points are:

- `src/storage/mod.rs`: raw storage evidence before EVMole collapses records;
- `src/compose/`: VSL matcher, semantic comparison, policy, and bias experiment;
- `tools/`: VSL generation from Solidity AST;
- `tests/fixtures/evmole/`: Solidity, runtime bytecode, VSL, and original
  EVMole output for each research case.

## Provenance

This repository began as a fork of
[EVMole](https://github.com/cdump/evmole) v0.9.3. The EVM interpreter,
selector/argument recovery, and storage tracer remain the analysis substrate;
Compose-specific validation is added as a separate host layer. The original
MIT license is retained in [LICENSE](./LICENSE).
