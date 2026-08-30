// SPDX-License-Identifier: MIT
pragma solidity >=0.8.30;

contract StorageKey {
    bytes32 constant ROOT_STORAGE_POSITION = keccak256("evmole.storage.key.root");
    bytes32 constant STORAGE_POSITION = keccak256("evmole.storage.key");
    bytes32 constant ROOT_STORAGE_2_POSITION = keccak256("evmole.storage.key.root.2");
    bytes32 constant STORAGE_2_POSITION = keccak256("evmole.storage.key.2");

    struct RootStorage {
        uint64 firstValue;
        uint256 firstSlot;
    }

    struct RootStorage2 {
        uint8 firstValue;
        uint128 firstSlot;
    }

    struct StorageKeyData {
        mapping(uint64 key => uint256 value) values;
        uint8[] smallValues;
        uint16[] mediumValues;
        bool flag;
        bool[5] flags;
        uint8[4] fixedValues;
    }

    struct StorageKeyData2 {
        mapping(uint8 key => uint128 value) values;
        uint64[] smallValues;
    }

    function getRootStorage() internal pure returns (RootStorage storage s) {
        bytes32 position = ROOT_STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }

    function getStorage() internal pure returns (StorageKeyData storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }

    function getRootStorage2() internal pure returns (RootStorage2 storage s) {
        bytes32 position = ROOT_STORAGE_2_POSITION;
        assembly {
            s.slot := position
        }
    }

    function getStorage2() internal pure returns (StorageKeyData2 storage s) {
        bytes32 position = STORAGE_2_POSITION;
        assembly {
            s.slot := position
        }
    }

    function readAll()
        external
        view
        returns (
            uint256 mappedValue,
            uint8 smallValue,
            uint16 mediumValue,
            bool flag,
            bool fixedFlag,
            uint8 fixedValue
        )
    {
        RootStorage storage root = getRootStorage();
        StorageKeyData storage s = getStorage();

        return (
            s.values[root.firstValue],
            s.smallValues[root.firstSlot],
            s.mediumValues[root.firstSlot],
            s.flag,
            s.flags[root.firstSlot % 5],
            s.fixedValues[root.firstSlot % 4]
        );
    }

    function readAll2() external view returns (uint128 mappedValue, uint64 smallValue) {
        RootStorage2 storage root = getRootStorage2();
        StorageKeyData2 storage s = getStorage2();

        return (s.values[root.firstValue], s.smallValues[root.firstSlot]);
    }
}
