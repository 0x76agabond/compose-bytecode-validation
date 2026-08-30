# VSL generator

`generate-vsl.mts` compiles one Solidity entrypoint with Foundry, reads its compact AST,
and writes the Compose virtual storage layout used by this PoC.

The VSL builder was copied from Compose CLI into `tools/vsl/`. The generator keeps its
readable `virtualPath`, while canonicalizing each record `id` through `cast keccak` so
the Rust bytecode matcher can compare it with physical EVM storage positions.

Run from this repository in Git Bash:

```sh
node ../../../Compose/cli/node_modules/tsx/dist/cli.mjs tools/generate-vsl.mts \
  tests/fixtures/evmole/1-normal/Normal.sol Normal
```

Use `--out path/to/vsl.json` to select another output file. `FOUNDRY_FORGE` and
`FOUNDRY_CAST` can override the executables when they are not already on `PATH`.

After generating fixture inputs, compare both Rust engines with:

```sh
cargo run --features fixture --example compose_fixture_comparison
```
