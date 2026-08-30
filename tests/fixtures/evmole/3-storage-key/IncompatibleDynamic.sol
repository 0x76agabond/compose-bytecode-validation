// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case3IncompatibleDynamic {
    bytes32 constant ROOT_POSITION = keccak256("compose.validation.case3.key-source");
    bytes32 constant STORAGE_POSITION = keccak256("compose.validation.case3.storage-key");

    struct KeySource { uint64 key; uint256 index; }
    struct Storage {
        mapping(uint64 => uint256) values;
        uint16[] smallValues;
        uint8[] mediumValues;
        bool flag;
        bool[5] flags;
        uint8[4] fixedValues;
    }

    function writeArrays(uint16 small, uint8 medium) external {
        uint256 index = _keySource().index;
        Storage storage s = _storage();
        s.smallValues[index] = small;
        s.mediumValues[index] = medium;
    }

    function _keySource() private pure returns (KeySource storage s) {
        bytes32 position = ROOT_POSITION;
        assembly { s.slot := position }
    }

    function _storage() private pure returns (Storage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly { s.slot := position }
    }
}
