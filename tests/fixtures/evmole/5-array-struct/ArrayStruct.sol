// SPDX-License-Identifier: MIT
pragma solidity >=0.8.30;

contract ArrayStruct {
    bytes32 constant ARRAY_STRUCT_STORAGE_POSITION = keccak256("evmole.array.struct");
    bytes32 constant ARRAY_STRUCT_2_STORAGE_POSITION = keccak256("evmole.array.struct.2");
    bytes32 constant ARRAY_STRUCT_MAPPING_STRUCT_STORAGE_POSITION =
        keccak256("evmole.array.struct.mapping.struct");
    bytes32 constant ARRAY_ADDRESS_STORAGE_POSITION = keccak256("evmole.array.address");

    struct Node {
        address target;
        bytes4 previousId;
        bytes8 nextId;
    }

    struct Node2 {
        address target;
        bytes8 nextId;
        bytes4 previousId;
    }

    struct ArrayStructStorage {
        Node[] nodes;
    }

    struct ArrayStructStorage2 {
        Node2[] nodes;
    }

    struct ArrayStructMappingStructStorage {
        Node[] arr;
        mapping(bytes4 selector => Node node) map;
    }

    struct AddressArrayStorage {
        address[] addresses;
    }

    function getArrayStructStorage() internal pure returns (ArrayStructStorage storage s) {
        bytes32 position = ARRAY_STRUCT_STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }

    function getArrayStructStorage2() internal pure returns (ArrayStructStorage2 storage s) {
        bytes32 position = ARRAY_STRUCT_2_STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }

    function getArrayStructMappingStructStorage() internal pure returns (ArrayStructMappingStructStorage storage s) {
        bytes32 position = ARRAY_STRUCT_MAPPING_STRUCT_STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }

    function getAddressArrayStorage() internal pure returns (AddressArrayStorage storage s) {
        bytes32 position = ARRAY_ADDRESS_STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }

    function readArrayStruct(uint256 index) external view returns (address target, bytes4 previousId, bytes8 nextId) {
        Node storage node = getArrayStructStorage().nodes[index];

        return (node.target, node.previousId, node.nextId);
    }

    function readArrayStruct2(uint256 index) external view returns (address target, bytes8 nextId, bytes4 previousId) {
        Node2 storage node = getArrayStructStorage2().nodes[index];

        return (node.target, node.nextId, node.previousId);
    }

    function readArrayStructMappingStruct(uint256 index, bytes4 selector)
        external
        view
        returns (
            address arrTarget,
            bytes4 arrPreviousId,
            bytes8 arrNextId,
            address mapTarget,
            bytes4 mapPreviousId,
            bytes8 mapNextId
        )
    {
        ArrayStructMappingStructStorage storage s = getArrayStructMappingStructStorage();
        Node storage arrNode = s.arr[index];
        Node storage mapNode = s.map[selector];

        return (
            arrNode.target,
            arrNode.previousId,
            arrNode.nextId,
            mapNode.target,
            mapNode.previousId,
            mapNode.nextId
        );
    }

    function readAddressArray(uint256 index) external view returns (address target) {
        return getAddressArrayStorage().addresses[index];
    }
}
