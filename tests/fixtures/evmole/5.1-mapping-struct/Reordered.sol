// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case51Reordered {
    bytes32 constant STORAGE_POSITION =
        keccak256("compose.validation.case5-1.mapping-struct");

    struct Node {
        address owner;
        uint256 amount;
    }

    struct Storage {
        mapping(uint256 => Node) nodes;
    }

    function writeNode(uint256 key, uint256 amount, address owner) external {
        Node storage node = _storage().nodes[key];
        node.owner = owner;
        node.amount = amount;
    }

    function _storage() private pure returns (Storage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }
}
