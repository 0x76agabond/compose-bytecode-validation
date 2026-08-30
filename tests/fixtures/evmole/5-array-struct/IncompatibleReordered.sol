// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case5IncompatibleReordered {
    bytes32 constant STORAGE_POSITION = keccak256("compose.validation.case5.array-struct");

    struct Node {
        address target;
        bytes8 nextId;
        bytes4 previousId;
    }

    struct Storage { Node[] nodes; }

    function writeNode(uint256 index, address target, bytes8 nextId, bytes4 previousId) external {
        Node storage node = _storage().nodes[index];
        node.target = target;
        node.nextId = nextId;
        node.previousId = previousId;
    }

    function _storage() private pure returns (Storage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly { s.slot := position }
    }
}
