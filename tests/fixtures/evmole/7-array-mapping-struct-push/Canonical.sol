// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case7Canonical {
    bytes32 constant STORAGE_POSITION = keccak256("compose.validation.case7.mapping-value");

    struct Node {
        address target;
        bytes4 previousId;
        bytes8 nextId;
    }

    struct Storage { mapping(bytes4 => Node[]) nodes; }

    function writeNode(bytes4 selector, address target, bytes4 previousId, bytes8 nextId) external {
        Node storage node = _storage().nodes[selector].push();
        node.target = target;
        node.previousId = previousId;
        node.nextId = nextId;
    }

    function _storage() private pure returns (Storage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly { s.slot := position }
    }
}
