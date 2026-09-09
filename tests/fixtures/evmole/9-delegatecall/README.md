# Delegatecall Diamond Fixture

This is a complete Foundry project with the Compose diamond router and its
normal facet set. `DelegateCallFacet` is first reached through the diamond's
storage-backed `DELEGATECALL` dispatcher, then exercises target recovery for a
second delegatecall. All storage-writing implementations use the existing
Compose router namespace:

`erc8042:erc8153.diamond`

| DelegateCallFacet case | Target source | Implementation | Expected result |
| --- | --- | --- | --- |
| `delegateSetFacet` | Immutable | `CompatibleDiamondDelegateImplementation` | Validated `FacetNode.facet` write |
| `delegateOverwriteFacet` | Immutable | `IncompatibleDiamondDelegateImplementation` | Collision: `uint256` overwrites `address` |
| `configureStoredTarget(bytes4)` + `delegateStoredSetFacet` | Persistent storage | `StoredDiamondDelegateImplementation` | Resolve target through `SLOAD`, then validate write |
| `delegateCalldataTarget` | Calldata | Caller supplied | Untraceable delegatecall warning |
| `delegateTransientTarget` | Transient storage | Caller supplied | Untraceable delegatecall warning |
| `delegateSymbolicTarget` | Symbolic hash expression | Derived at runtime | Untraceable delegatecall warning |
| `delegateEmptyTarget` | Constant | Empty account code | Untraceable delegatecall warning |
| `delegateMissingSelector` | Immutable | `MissingSelectorDelegateImplementation` | Warning: recovered selector is absent |
| `delegateNestedSetFacet` | Immutable, then immutable | `NestedDelegateImplementation` -> `FinalDiamondDelegateImplementation` | Recursive validated write |

`CanonicalDiamondStorage.sol` supplies the one-record canonical VSL. The
integration test deploys a real diamond, invokes both writes through its
fallback, and asserts the diamond's physical storage changes. The validator
traces `DelegateCallFacet` with the deployed diamond supplied as its storage
context.
