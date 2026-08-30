// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case3Canonical {
    bytes32 constant ROOT_POSITION = keccak256("compose.validation.case3.key-source");
    bytes32 constant STORAGE_POSITION = keccak256("compose.validation.case3.storage-key");

    struct KeySource {
        uint64 key;
        uint256 index;
    }

    struct Storage {
        mapping(uint64 => uint256) values;
        uint8[] smallValues;
        uint16[] mediumValues;
        bool flag;
        bool[5] flags;
        uint8[4] fixedValues;
    }

    function writeKeySource(uint64 key, uint256 index) external {
        KeySource storage source = _keySource();
        source.key = key;
        source.index = index;
    }

    function writeAll(uint256 mapped, uint8 small, uint16 medium, bool flag, bool fixedFlag, uint8 fixedValue)
        external
    {
        KeySource storage source = _keySource();
        Storage storage s = _storage();
        s.values[source.key] = mapped;
        s.smallValues[source.index] = small;
        s.mediumValues[source.index] = medium;
        s.flag = flag;
        s.flags[source.index % 5] = fixedFlag;
        s.fixedValues[source.index % 4] = fixedValue;
    }

    function _keySource() private pure returns (KeySource storage s) {
        bytes32 position = ROOT_POSITION;
        assembly {
            s.slot := position
        }
    }

    function _storage() private pure returns (Storage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }
}
