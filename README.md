# Compose Bytecode Validation

Compose's Rust bytecode storage validator. It validates a facet's deployed
runtime bytecode against its Solidity-derived Virtual Storage Layout (VSL).

This is not a general-purpose Solidity decompiler and it does not attempt to
reconstruct a complete contract layout from bytecode. Compose already has a
stronger source-side model: the VSL knows declared types, packing, container
boundaries, and named storage roots. This project asks a narrower question:

> Does bytecode storage evidence clearly contradict the selected facet's VSL?

## Virtual Storage Layout

The VSL is the canonical source-side model of the full diamond's declared
storage. It is generated from Solidity's compact AST before this validator sees
any bytecode. A record has a namespace-derived root identity and a compact
layout encoding, while its readable virtual path describes the variable or
virtual struct child that occupies each position.

The layout is a small recursive token stream:

- `0x01..0x73`: one byte per scalar type, for example `0x01` is `bool` and
  `0x03` is `address`;
- `0xf1`: mapping;
- `0xf2`: dynamic array;
- `0xf3`: fixed array;
- `0xf4`: struct; and
- `0xff`: end of the current container.

For example, this Solidity shape:

```solidity
struct Node {
    address target;
    bytes4 previousId;
    bytes8 nextId;
}

struct Storage {
    mapping(bytes4 => Node[]) nodes;
}
```

uses two records. The root record is:

```text
0xf1 0x53 0xf2 0xff
mapping bytes4 dynamic-array virtual-child
```

The child record at `nodes.0` holds the `Node` schema:

```text
0x03 0x53 0x57
address bytes4 bytes8
```

Here `0xf2 0xff` means the dynamic-array element is a virtual child rather
than an inline type. `0xf2` is already the mapping value, so that one `0xff`
finishes the array branch and the mapping is complete too; it does not close an
unrelated outer struct. The separate child record preserves the struct schema
and physical span.

It preserves the Solidity rules that matter for bytecode validation:

- declaration order, slot boundaries, byte packing, and fixed-array physical
  span;
- semantic boundaries for structs, mappings, and dynamic arrays, including
  virtual child records for structs inside containers; and
- declared scalar type and width at every comparable slot and byte offset.

That gives the validator a complete expected storage map for the diamond rather
than asking a decompiler to rediscover one from every facet in isolation. VSL
does not prove that bytecode reaches every path or interpret arbitrary assembly;
it supplies the expected coordinate system and the type/packing constraints
against which recovered persistent writes are challenged.

## Inputs and Verdicts

Each validation run consumes:

- a facet's runtime bytecode;
- VSL records generated from the Solidity compact AST; and
- concrete root slots derived from the VSL namespaces.

The validator traces persistent `SSTORE` operations and compares concrete
storage evidence with the canonical full-diamond VSL. It reports four evidence
collections:

| Evidence | Meaning |
| --- | --- |
| `collisions` | A persistent write contradicts a VSL slot, offset, width, or value/container shape. |
| `validatedVariables` | A recovered persistent write matches one VSL variable. |
| `uncertainScopes` | A write has a concrete storage scope but cannot be compared conclusively. |
| `diagnostics` | The engine cannot recover even a concrete persistent storage root. |

No collision is not a complete proof of safety. Unreached paths and unsupported
compiler patterns remain outside the evidence set.

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
variable that a facet could access. Mapping key-type differences are diagnostic
only; value shape, container shape, byte width, packing offset, and root slot
identity are storage compatibility evidence.

## Fixture Coverage

The nine fixtures in `tests/fixtures/evmole/` are write-based challenge suites.
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
| `5-array-struct` | Packed and multi-slot array structs, reordered members, and adjacent arrays | Canonical child paths validate; reordered fields and an incompatible element stride collide. |
| `6-array-mapping-struct` | Indexed mapping to an array of packed structs | Canonical indexed writes validate; reordered members collide without uncertainty. |
| `7-array-mapping-struct-push` | Mapping to an array of packed structs created with `push()`, including a nested dynamic array variant | Recursive paths validate; incompatible nested containers and extra fields collide, with scoped uncertainty where `push()` loses a child boundary. |
| `final-full-storage` | Full representative VSL: packed primitives, inline structs, mappings, arrays, fixed arrays, struct containers, and independent ERC-8110-style domains | Canonical writes validate across 30 recovered variables; incompatible terminal, nested, and dynamic-key writes collide. |
| `8-bytes-string` | `bytes`, `string`, `bytes[]`, and `string[]` assignment and append flows | Solidity encodes `bytes` and `string` identically in storage, so their type swap is not a physical collision. Known tracer limitation: canonical array appends currently produce false collisions and scoped uncertainty. |

An inferred fallback `uint256` cannot prove a collision. The raw tracer marks
whether the write value type was actually recovered; fallback values are
reported only as scoped uncertainty. Dynamic-array length writes are compared
as container metadata rather than as element writes.

### Current Result Snapshot

The current assertion-backed run covers 24 contracts across the nine fixture
families:

| Fixture | Variant | Collisions | Validated | Scoped uncertainty |
| --- | --- | ---: | ---: | ---: |
| `1-normal` | canonical | 0 | 11 | 0 |
| `1-normal` | incompatible packed width | 2 | 3 | 0 |
| `2-constant-key` | canonical | 0 | 4 | 0 |
| `2-constant-key` | incompatible dynamic width | 1 | 3 | 0 |
| `3-storage-key` | canonical | 0 | 8 | 0 |
| `3-storage-key` | incompatible dynamic width | 2 | 0 | 0 |
| `3-storage-key` | incompatible fixed width | 2 | 0 | 0 |
| `4-mapping-struct` | canonical | 0 | 5 | 0 |
| `4-mapping-struct` | incompatible reordered members | 2 | 3 | 0 |
| `5-array-struct` | canonical | 0 | 9 | 0 |
| `5-array-struct` | incompatible reordered members | 2 | 1 | 0 |
| `5-array-struct` | incompatible wide reordered members | 2 | 1 | 0 |
| `5-array-struct` | incompatible address array | 0 | 1 | 0 |
| `5-array-struct` | adjacent arrays | 1 | 1 | 0 |
| `6-array-mapping-struct` | canonical indexed mapping-array-struct | 0 | 3 | 0 |
| `6-array-mapping-struct` | incompatible indexed member order | 3 | 0 | 0 |
| `7-array-mapping-struct-push` | canonical mapping-array-struct | 0 | 4 | 0 |
| `7-array-mapping-struct-push` | incompatible mapping array struct | 1 | 1 | 1 |
| `7-array-mapping-struct-push` | incompatible mapping array and fields | 3 | 1 | 1 |
| `final-full-storage` | canonical full storage | 0 | 30 | 0 |
| `final-full-storage` | compatible prefix storage | 0 | 1 | 0 |
| `final-full-storage` | incompatible full storage | 12 | 4 | 1 |
| `8-bytes-string` | canonical bytes and string | 31 | 2 | 4 |
| `8-bytes-string` | swapped bytes/string types | 31 | 2 | 4 |

All variants currently complete without an unresolved-root diagnostic. The
engine reliably tracks root slots, static slot shifts, selectors, program
counters, constant mapping keys, and storage-derived mapping keys.

VSL-derived trace hints now recover mapping key types for constant and
storage-loaded keys. Packed read-modify-write values retain their ABI type
through dynamic index shifting, including Solidity's boolean normalization.

Mapping-value structs retain constant child-slot deltas from `KECCAK256 +
constant` and packed write masks. The storage tracer also preserves ordered
path segments for mappings, dynamic arrays, and slot offsets. The validator
walks those segments recursively through VSL virtual struct children, so cases
4 through 7 use the same path-matching mechanism rather than case-specific
rules. This validates packed and multi-slot array-struct members, detects
element-stride contradictions, and detects nested dynamic containers or fields
that exceed the canonical child struct span.

Some writes remain deliberately scoped uncertainty when the tracer knows a
concrete storage position but loses the nested child path or value type. In the
case 7 `push()` incompatible variants, an inner array-length update shares the canonical
first-field position after the tracer loses its element index, so it cannot
prove a contradiction by itself. This does not suppress independently recovered
member writes or their collisions.

The full-storage fixture keeps its original `compose.fixture.virtual-storage`
domain and appends four ERC-8110-style independent domains:
`terminal.v1`, `nested.v1`, `dynamic-keys.v1`, and `dynamic-data.v1`. They
exercise terminal struct-member ambiguity, mapping-to-dynamic-array-to-struct
paths, dynamic `bytes`/`string` mapping keys, and dynamic `bytes`/`string`
value writes. A source type difference is not itself a collision: writing
`uint256` through `mapping(uint256 => uint256)` is accepted when it targets the
canonical `Node.amount` member, while an `address` write at that same position
is a collision.

Fixture 8 is intentionally not a passing validation claim. `bytes` and
`string` share a physical encoding, so exchanging them is not itself a storage
collision. More importantly, the current tracer mis-reconstructs append paths
for `bytes[]` and `string[]`, producing false collisions even for canonical
bytecode. The fixture preserves that behavior as a bounded hardening target.

## Run the Validator

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
From this repository, with a Compose CLI checkout available at the sibling
path used below:

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
  sources for each validation case.

## Provenance

This repository began as a fork of
[EVMole](https://github.com/cdump/evmole) v0.9.3. The EVM interpreter,
selector/argument recovery, and storage tracer remain the analysis substrate;
Compose-specific validation is added as a separate host layer. The original
MIT license is retained in [LICENSE](./LICENSE).
