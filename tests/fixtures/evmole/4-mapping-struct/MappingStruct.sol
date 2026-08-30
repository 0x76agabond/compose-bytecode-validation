// SPDX-License-Identifier: MIT
pragma solidity >=0.8.30;

contract MappingStruct {
    bytes32 constant MAPPING_STRUCT_STORAGE_POSITION = keccak256("evmole.mapping.struct");
    bytes32 constant MAPPING_STRUCT_2_STORAGE_POSITION = keccak256("evmole.mapping.struct.2");

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

    struct MappingStructStorage {
        mapping(bytes4 selector => Node node) nodes;
    }

    struct MappingStructStorage2 {
        mapping(bytes4 selector => Node2 node) nodes;
    }

    function getMappingStructStorage() internal pure returns (MappingStructStorage storage s) {
        bytes32 position = MAPPING_STRUCT_STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }

    function getMappingStructStorage2() internal pure returns (MappingStructStorage2 storage s) {
        bytes32 position = MAPPING_STRUCT_2_STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }

    function readMappingStruct(bytes4 selector) external view returns (address target, bytes4 previousId, bytes8 nextId) {
        Node storage node = getMappingStructStorage().nodes[selector];

        return (node.target, node.previousId, node.nextId);
    }

    function readMappingStruct2(bytes4 selector) external view returns (address target, bytes8 nextId, bytes4 previousId) {
        Node2 storage node = getMappingStructStorage2().nodes[selector];

        return (node.target, node.nextId, node.previousId);
    }
}
