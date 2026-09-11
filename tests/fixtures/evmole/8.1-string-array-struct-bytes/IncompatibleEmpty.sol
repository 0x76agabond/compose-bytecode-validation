// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case81IncompatibleStringEmpty {
    struct Storage {
        string[] values;
    }

    bytes32 private constant STORAGE_POSITION =
        keccak256("compose.validation.case8-1.string-array-struct-bytes");

    function appendEmpty() external {
        _storage().values.push();
    }

    function _storage() private pure returns (Storage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }
}
