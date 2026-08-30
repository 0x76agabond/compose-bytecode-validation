// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case5Canonical {
    bytes32 constant STORAGE_POSITION = keccak256("compose.validation.case5.array-struct");

    struct Node {
        address target;
        bytes4 previousId;
        bytes8 nextId;
    }

    struct Storage { Node[] nodes; }

    function writeNode(uint256 index, address target, bytes4 previousId, bytes8 nextId) external {
        Node storage node = _storage().nodes[index];
        node.target = target;
        node.previousId = previousId;
        node.nextId = nextId;
    }

    function _storage() private pure returns (Storage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly { s.slot := position }
    }
}
