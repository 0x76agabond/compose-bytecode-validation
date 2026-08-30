// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case5IncompatibleAddress {
    bytes32 constant STORAGE_POSITION = keccak256("compose.validation.case5.array-struct");

    struct Storage { address[] nodes; }

    function writeNode(uint256 index, address target) external {
        _storage().nodes[index] = target;
    }

    function _storage() private pure returns (Storage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly { s.slot := position }
    }
}
