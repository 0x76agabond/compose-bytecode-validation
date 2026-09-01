// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case4Incompatible {
    bytes32 constant STORAGE_POSITION = keccak256("compose.validation.case4.mapping-struct");

    struct Node {
        address target;
        bytes8 nextId;
        bytes4 previousId;
        uint timestamp;
        uint256 timestamp2;
    }

    struct Storage { mapping(bytes4 => Node) nodes; }

    function writeNode(bytes4 selector, address target, bytes4 previousId, bytes8 nextId) external {
        Node storage node = _storage().nodes[selector];
        node.nextId = nextId;
        node.previousId = previousId;
        node.target = target;
        node.timestamp = block.timestamp;
        node.timestamp2 = block.timestamp;
    }

    function _storage() private pure returns (Storage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly { s.slot := position }
    }
}
