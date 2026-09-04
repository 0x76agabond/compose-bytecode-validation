// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case6Incompatible {
    bytes32 constant STORAGE_POSITION = keccak256("compose.validation.case6.mapping-array-struct");

    struct Node {
        bytes8 nextId;
        address target;
        bytes4 previousId;
    }

    struct Storage { mapping(bytes4 => Node[]) nodes; }

    function writeNode(
        bytes4 selector,
        uint256 index,
        address target,
        bytes4 previousId,
        bytes8 nextId
    ) external {
        Node storage node = _storage().nodes[selector][index];
        node.target = target;
        node.previousId = previousId;
        node.nextId = nextId;
    }

    function _storage() private pure returns (Storage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly { s.slot := position }
    }
}
