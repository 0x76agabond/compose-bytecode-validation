// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case81CanonicalWithData {
    /**
     * @custom:storage-location erc8042:compose.validation.case8-1.string-array-struct-bytes
     */
    struct Storage {
        Node[] values;
    }

    struct Node {
        bytes data;
    }

    bytes32 private constant STORAGE_POSITION =
        keccak256("compose.validation.case8-1.string-array-struct-bytes");

    function append(bytes calldata data) external {
        Node storage node = _storage().values.push();
        node.data = data;
    }

    function _storage() private pure returns (Storage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }
}
