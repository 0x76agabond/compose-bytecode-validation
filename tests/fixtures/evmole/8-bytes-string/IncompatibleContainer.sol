// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case8IncompatibleContainer {
    /**
     * @custom:storage-location erc8042:compose.validation.case8.bytes-string
     */
    struct Storage {
        bytes data;
        string text;
        mapping(uint256 => uint256) byteArray;
        mapping(uint256 => uint256) stringArray;
    }

    bytes32 private constant STORAGE_POSITION =
        keccak256("compose.validation.case8.bytes-string");

    function writeMappings(uint256 key, uint256 value) external {
        Storage storage s = _storage();
        s.byteArray[key] = value;
        s.stringArray[key] = value;
    }

    function _storage() private pure returns (Storage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }
}
