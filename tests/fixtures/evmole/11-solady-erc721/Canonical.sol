// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

/// @dev Exercises Solady's ERC721-style custom coordinate system:
///      keccak256(id[0:28] || seed) + id + id for ownership and
///      keccak256(address || seed) for balances.
contract Case11SoladyCanonical {
    /**
     * @custom:storage-location erc8042:compose.validation.case11.solady-erc721
     */
    struct Storage {
        mapping(uint256 => Ownership) ownerships;
        mapping(address => uint256) balances;
    }

    struct Ownership {
        address owner;
        uint96 extra;
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
            sstore(ownershipSlot, or(shl(160, extra), to))

            mstore(0x0c, seed)
            mstore(0x00, to)
            let balanceSlot := keccak256(0x0c, 0x1c)
            sstore(balanceSlot, add(sload(balanceSlot), 1))
        }
    }
}
