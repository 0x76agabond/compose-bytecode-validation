// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case51TerminalProjection {
    bytes32 constant STORAGE_POSITION =
        keccak256("compose.validation.case5-1.mapping-struct");

    struct Storage {
        mapping(uint256 => uint256) nodes;
    }

    function writeAmount(uint256 key, uint256 amount) external {
        _storage().nodes[key] = amount;
    }

    function _storage() private pure returns (Storage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }
}
