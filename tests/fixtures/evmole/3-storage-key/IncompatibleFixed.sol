// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case3IncompatibleFixed {
    bytes32 constant ROOT_POSITION = keccak256("compose.validation.case3.key-source");
    bytes32 constant STORAGE_POSITION = keccak256("compose.validation.case3.storage-key");

    struct KeySource { uint64 key; uint256 index; }
    struct Storage {
        mapping(uint64 => uint256) values;
        uint8[] smallValues;
        uint16[] mediumValues;
        bool flag;
        uint8[5] flags;
        bool[4] fixedValues;
    }

    function writeFixed(uint8 fixedFlag, bool fixedValue) external {
        uint256 index = _keySource().index;
        Storage storage s = _storage();
        s.flags[index % 5] = fixedFlag;
        s.fixedValues[index % 4] = fixedValue;
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
