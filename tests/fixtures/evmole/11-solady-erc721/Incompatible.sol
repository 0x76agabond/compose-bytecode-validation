// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case11SoladyIncompatible {
    struct Storage {
        mapping(uint256 => Ownership) ownerships;
        mapping(address => uint256) balances;
    }

    struct Ownership {
        uint96 extra;
        address owner;
    }

    bytes32 private constant STORAGE_POSITION =
        keccak256("compose.validation.case11.solady-erc721");
    uint32 private constant MASTER_SLOT_SEED = uint32(uint256(STORAGE_POSITION));

    function mintLike(uint256 id, address to, uint96 extra) external {
        uint32 seed = MASTER_SLOT_SEED;
        assembly {
            to := shr(96, shl(96, to))
            mstore(0x00, id)
            mstore(0x1c, seed)
            let ownershipSlot := add(id, add(id, keccak256(0x00, 0x20)))
            sstore(ownershipSlot, or(shl(160, to), extra))

            mstore(0x0c, seed)
            mstore(0x00, to)
            let balanceSlot := keccak256(0x0c, 0x1c)
            sstore(balanceSlot, add(sload(balanceSlot), 1))
        }
    }
}
