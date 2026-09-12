// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case11SoladyIncompatible {
    struct OwnershipStorage {
        mapping(uint256 => Ownership) ownerships;
    }

    struct BalanceStorage {
        mapping(address => Account) accounts;
    }

    struct OperatorStorage {
        mapping(address => mapping(address => address)) approvals;
    }

    struct Ownership {
        uint96 extra;
        address owner;
        uint256 approved;
    }

    struct Account {
        uint224 aux;
        uint32 balance;
    }

    bytes32 private constant OWNERSHIP_POSITION =
        keccak256("compose.validation.case11.solady.ownership");
    bytes32 private constant BALANCE_POSITION =
        keccak256("compose.validation.case11.solady.balance");
    bytes32 private constant OPERATOR_POSITION =
        keccak256("compose.validation.case11.solady.operator");
    bytes32 private constant OWNERSHIP_SEED =
        bytes32(uint256(uint64(uint256(OWNERSHIP_POSITION))) << 192);
    bytes32 private constant BALANCE_SEED =
        bytes32(uint256(uint64(uint256(BALANCE_POSITION))) << 192);
    uint64 private constant OPERATOR_SEED_MASKED =
        uint64(uint256(OPERATOR_POSITION)) << 32;

    function writeOwnership(
        uint256 id,
        address owner,
        uint96 extra,
        address approved
    ) external {
        bytes32 seed = OWNERSHIP_SEED;
        assembly {
            owner := shr(96, shl(96, owner))
            approved := shr(96, shl(96, approved))
            mstore(0x00, id)
            mstore(0x1c, seed)
            let ownershipSlot := add(id, add(id, keccak256(0x00, 0x20)))
            sstore(ownershipSlot, or(shl(160, owner), extra))
            sstore(add(ownershipSlot, 1), approved)
        }
    }

    function writeBalance(address owner, uint32 accountBalance, uint224 aux) external {
        bytes32 seed = BALANCE_SEED;
        assembly {
            owner := shr(96, shl(96, owner))
            mstore(0x1c, seed)
            mstore(0x00, owner)
            let balanceSlot := keccak256(0x0c, 0x1c)
            sstore(balanceSlot, or(shl(32, accountBalance), aux))
        }
    }

    function setOperator(address owner, address operator, bool approved) external {
        uint64 seed = OPERATOR_SEED_MASKED;
        assembly {
            owner := shr(96, shl(96, owner))
            operator := shr(96, shl(96, operator))
            approved := iszero(iszero(approved))
            mstore(0x1c, operator)
            mstore(0x08, seed)
            mstore(0x00, owner)
            sstore(keccak256(0x0c, 0x30), approved)
        }
    }
}
