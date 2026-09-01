// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case6IncompatibleOnlyArray {
    bytes32 constant STORAGE_POSITION = keccak256("compose.validation.case6.mapping-value");

    struct Node { uint256[] values; }
    struct Storage { mapping(bytes4 => Node[]) nodes; }

    function appendValue(bytes4 selector, uint256 value) external {
        _storage().nodes[selector].push().values.push(value);
    }

    function _storage() private pure returns (Storage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly { s.slot := position }
    }
}
