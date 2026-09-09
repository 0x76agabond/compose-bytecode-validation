// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

/// @dev Exercises a second delegatecall after Compose routes through the diamond.
contract DelegateCallFacet {
    bytes4 private constant STORED_TARGET_KEY = 0xdecafbad;

    struct FacetNode {
        address facet;
        bytes4 prevFacetNodeId;
        bytes4 nextFacetNodeId;
    }

    struct DiamondStorage {
        mapping(bytes4 functionSelector => FacetNode) facetNodes;
        bytes4 headFacetNodeId;
        bytes4 tailFacetNodeId;
        uint32 facetCount;
        uint32 selectorCount;
    }

    bytes32 private constant DIAMOND_STORAGE_POSITION = keccak256("erc8153.diamond");
    address private constant EMPTY_TARGET = address(0x000000000000000000000000000000000000bEEF);

    address private immutable COMPATIBLE_IMPLEMENTATION;
    address private immutable INCOMPATIBLE_IMPLEMENTATION;
    address private immutable STORED_IMPLEMENTATION;
    address private immutable NESTED_IMPLEMENTATION;
    address private immutable MISSING_SELECTOR_IMPLEMENTATION;

    constructor(
        address compatible_,
        address incompatible_,
        address stored_,
        address nested_,
        address missingSelector_
    ) {
        COMPATIBLE_IMPLEMENTATION = compatible_;
        INCOMPATIBLE_IMPLEMENTATION = incompatible_;
        STORED_IMPLEMENTATION = stored_;
        NESTED_IMPLEMENTATION = nested_;
        MISSING_SELECTOR_IMPLEMENTATION = missingSelector_;
    }

    function delegateSetFacet(bytes4, address) external {
        _delegate(COMPATIBLE_IMPLEMENTATION);
    }

    function delegateOverwriteFacet(bytes4, uint256) external {
        _delegate(INCOMPATIBLE_IMPLEMENTATION);
    }

    function configureStoredTarget(bytes4 targetKey) external {
        _storage().facetNodes[targetKey].facet = STORED_IMPLEMENTATION;
    }

    function delegateStoredSetFacet(bytes4, address) external {
        _delegate(_storage().facetNodes[STORED_TARGET_KEY].facet);
    }

    function delegateCalldataTarget(address target) external {
        _delegate(target);
    }

    function delegateTransientTarget(address target) external {
        assembly ("memory-safe") {
            tstore(0, target)
            let transientTarget := tload(0)
            calldatacopy(0, 0, calldatasize())
            let success := delegatecall(gas(), transientTarget, 0, calldatasize(), 0, 0)
            returndatacopy(0, 0, returndatasize())
            switch success
            case 0 { revert(0, returndatasize()) }
            default { return(0, returndatasize()) }
        }
    }

    function delegateSymbolicTarget() external {
        assembly ("memory-safe") {
            mstore(0, caller())
            let target := and(keccak256(0, 32), 0xffffffffffffffffffffffffffffffffffffffff)
            calldatacopy(0, 0, calldatasize())
            let success := delegatecall(gas(), target, 0, calldatasize(), 0, 0)
            returndatacopy(0, 0, returndatasize())
            switch success
            case 0 { revert(0, returndatasize()) }
            default { return(0, returndatasize()) }
        }
    }

    function delegateEmptyTarget() external {
        _delegate(EMPTY_TARGET);
    }

    function delegateMissingSelector() external {
        _delegate(MISSING_SELECTOR_IMPLEMENTATION);
    }

    function delegateNestedSetFacet(bytes4, address) external {
        _delegate(NESTED_IMPLEMENTATION);
    }

    function exportSelectors() external pure returns (bytes memory) {
        return abi.encodePacked(
            this.delegateSetFacet.selector,
            this.delegateOverwriteFacet.selector,
            this.configureStoredTarget.selector,
            this.delegateStoredSetFacet.selector,
            this.delegateCalldataTarget.selector,
            this.delegateTransientTarget.selector,
            this.delegateSymbolicTarget.selector,
            this.delegateEmptyTarget.selector,
            this.delegateMissingSelector.selector,
            this.delegateNestedSetFacet.selector
        );
    }

    function _storage() private pure returns (DiamondStorage storage s) {
        bytes32 position = DIAMOND_STORAGE_POSITION;
        assembly ("memory-safe") {
            s.slot := position
        }
    }

    function _delegate(address target) private {
        assembly ("memory-safe") {
            calldatacopy(0, 0, calldatasize())
            let success := delegatecall(gas(), target, 0, calldatasize(), 0, 0)
            returndatacopy(0, 0, returndatasize())
            switch success
            case 0 { revert(0, returndatasize()) }
            default { return(0, returndatasize()) }
        }
    }
}
