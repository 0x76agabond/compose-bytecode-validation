// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case6IncompatibleArrayAndFields {
    bytes32 constant STORAGE_POSITION = keccak256("compose.validation.case6.mapping-value");

    struct Node {
        uint256[] values;
        address target;
        bool enabled;
    }

    struct Storage { mapping(bytes4 => Node[]) nodes; }

    function writeNode(bytes4 selector, uint256 value, address target, bool enabled) external {
        Node storage node = _storage().nodes[selector].push();
        node.values.push(value);
        node.target = target;
        node.enabled = enabled;
    }

    function _storage() private pure returns (Storage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly { s.slot := position }
    }
}
