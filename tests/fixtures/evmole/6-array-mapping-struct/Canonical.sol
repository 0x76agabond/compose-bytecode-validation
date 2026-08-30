// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case6Canonical {
    bytes32 constant STORAGE_POSITION = keccak256("compose.validation.case6.mapping-value");

    struct Storage { mapping(bytes4 => address) nodes; }

    function writeNode(bytes4 selector, address target) external {
        _storage().nodes[selector] = target;
    }

    function _storage() private pure returns (Storage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly { s.slot := position }
    }
}
