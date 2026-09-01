// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case5IncompatibleWideReordered {
    bytes32 constant STORAGE_POSITION = keccak256("compose.validation.case5.array-struct");

    struct Node {
        address target;
        bytes4 previousId;
        bytes8 nextId;
    }

    struct WideNode {
        uint256 value;
        uint256 timestamp;
        address target;
    }

    struct Storage {
        Node[] nodes;
        WideNode[] wideNodes;
    }

    function writeWideNode(
        uint256 index,
        uint256 value,
        address target,
        uint256 timestamp
    ) external {
        WideNode storage node = _storage().wideNodes[index];
        node.value = value;
        node.target = target;
        node.timestamp = timestamp;
    }

    function _storage() private pure returns (Storage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly { s.slot := position }
    }
}
