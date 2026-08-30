// SPDX-License-Identifier: MIT
pragma solidity >=0.8.30;

contract ArrayMappingStruct {
    bytes32 constant STORAGE_POSITION = keccak256("evmole.mapping.struct.array.bool");
    bytes32 constant ONLY_ARRAY_STORAGE_POSITION = keccak256("evmole.mapping.struct.only.array");
    bytes32 constant RAW_ADDRESS_STORAGE_POSITION = keccak256("evmole.mapping.address");

    struct NodeWithArray {
        uint256[] values;
        address target;        
        bool enabled;
    }

    struct NodeOnlyArray {
        uint256[] values;
    }

    struct ArrayMappingStructStorage {
        mapping(bytes4 selector => NodeWithArray node) nodes;
    }

    struct OnlyArrayStorage {
        mapping(bytes4 selector => NodeOnlyArray node) nodes;
    }

    struct RawAddressStorage {
        mapping(bytes4 selector => address target) nodes;
    }

    function getStorage() internal pure returns (ArrayMappingStructStorage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }

    function getOnlyArrayStorage() internal pure returns (OnlyArrayStorage storage s) {
        bytes32 position = ONLY_ARRAY_STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }

    function getRawAddressStorage() internal pure returns (RawAddressStorage storage s) {
        bytes32 position = RAW_ADDRESS_STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }

    function readAll(bytes4 selector, uint256 index) external view returns (address target, uint256 value, bool enabled) {
        NodeWithArray storage record = getStorage().nodes[selector];

        return (record.target, record.values[index], record.enabled);
    }

    function readOnlyArray(bytes4 selector, uint256 index) external view returns (uint256 value) {
        return getOnlyArrayStorage().nodes[selector].values[index];
    }

    function readRawAddress(bytes4 selector) external view returns (address target) {
        return getRawAddressStorage().nodes[selector];
    }
}
