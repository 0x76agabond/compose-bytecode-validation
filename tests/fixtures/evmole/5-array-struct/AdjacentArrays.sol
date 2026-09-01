// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case5AdjacentArrays {
    bytes32 constant STORAGE_POSITION = keccak256("compose.validation.case5.array-struct");

    struct Storage {
        address[] targets;
        uint256[] values;
    }

    function writeNode(uint256 index, address target, uint256 value) external {
        Storage storage s = _storage();
        s.targets[index] = target;
        s.values[index] = value;
    }

    function _storage() private pure returns (Storage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly { s.slot := position }
    }
}
