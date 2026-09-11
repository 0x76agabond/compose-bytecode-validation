// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case10AssemblyCanonical {
    /**
     * @custom:storage-location erc8042:compose.validation.case10.total-assembly
     */
    struct Storage {
        uint256 amount;
        address owner;
        bool active;
        mapping(uint256 => uint256) balances;
        uint256[] scores;
    }

    bytes32 private constant STORAGE_POSITION =
        keccak256("compose.validation.case10.total-assembly");

    function writePrimitives(uint256 amount, address owner, bool active) external {
        bytes32 root = STORAGE_POSITION;
        assembly {
            sstore(root, amount)
            sstore(add(root, 1), or(owner, shl(160, active)))
        }
    }

    function writeBalance(uint256 key, uint256 value) external {
        bytes32 root = STORAGE_POSITION;
        assembly {
            mstore(0x00, key)
            mstore(0x20, add(root, 2))
            sstore(keccak256(0x00, 0x40), value)
        }
    }

    function appendScore(uint256 value) external {
        bytes32 root = STORAGE_POSITION;
        assembly {
            let arraySlot := add(root, 3)
            let length := sload(arraySlot)
            mstore(0x00, arraySlot)
            let dataStart := keccak256(0x00, 0x20)
            sstore(add(dataStart, length), value)
            sstore(arraySlot, add(length, 1))
        }
    }

    function writeRaw(bytes32 slot, uint256 value) external {
        assembly {
            sstore(slot, value)
        }
    }
}
